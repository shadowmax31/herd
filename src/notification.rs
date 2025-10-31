use crate::error::{Error, Result};
use chrono::{DateTime, Datelike, Duration, Local, NaiveTime};
use rusqlite::Connection;

use crate::day;

#[derive(Debug)]
pub struct Notification {
    id: usize,
    title: String,
    message: String,
    hour: u8,
    minute: u8,
    day: u8,
}

impl Notification {
    pub fn notify_now(message: String) -> Result<()> {
        Notification::new(0, "Herd".to_string(), message, "00:00".to_string(), 0)?.run(false)?;
        Ok(())
    }

    pub fn error_now(message: String) -> Result<()> {
        Notification::new(0, "Herd".to_string(), message, "00:00".to_string(), 0)?.run(true)?;
        Ok(())
    }

    pub fn new(id: usize, title: String, message: String, time: String, day: u8) -> Result<Self> {
        let hour_minute: Vec<&str> = time.split(":").collect();

        let hour: u8 = hour_minute
            .first()
            .ok_or(Error::Time(time.to_string()))?
            .parse()?;
        let minute: u8 = hour_minute
            .get(1)
            .ok_or(Error::Time(time.to_string()))?
            .parse()?;

        let n = Notification {
            id,
            title,
            message,
            hour,
            minute,
            day,
        };

        Ok(n)
    }

    pub fn get_title_len(&self) -> usize {
        self.title.len()
    }

    pub fn get_id_len(&self) -> usize {
        self.id.to_string().len()
    }

    pub fn simple_print(
        &self,
        now: &DateTime<Local>,
        id_width: usize,
        title_width: usize,
    ) -> Result<()> {
        println!(
            "{:>id_width$}: {:<title_width$} | Next: {} | Days: `{}`",
            self.id,
            self.title,
            self.next(now)?.format("%Y-%m-%d %H:%M"),
            day::to_string(self.day),
            id_width = id_width,
            title_width = title_width,
        );

        Ok(())
    }

    pub fn find_all(connection: &Connection) -> Result<Vec<Notification>> {
        let mut statement = connection.prepare("SELECT * FROM notification")?;

        let notification_iter = statement.query_map([], |row| {
            let notification = Notification::new(
                row.get("id")?,
                row.get("title")?,
                row.get("message")?,
                row.get("time")?,
                row.get("day")?,
            );

            Ok(notification)
        })?;

        let mut list = vec![];
        for notification in notification_iter {
            list.push(notification??);
        }

        Ok(list)
    }

    pub fn insert(
        connection: &Connection,
        title: String,
        message: String,
        time: String,
        day: u8,
    ) -> Result<()> {
        connection.execute(
            "INSERT INTO notification (title, message, time, day) values (?1, ?2, ?3, ?4)",
            (title, message, time, day),
        )?;
        Ok(())
    }

    pub fn run(&self, err: bool) -> Result<()> {
        let now = chrono::offset::Local::now();

        let mut c = std::process::Command::new("notify-send");
        c.arg(&self.title).arg(format!(
            "{} \n\n{}",
            &self.message,
            now.format("%Y-%m-%d %H:%M:%S"),
        ));

        if err {
            c.arg("-u").arg("critical");
        }

        c.spawn()?;

        std::process::Command::new("play")
            .arg("-q")
            .arg("/usr/share/sounds/freedesktop/stereo/bell.oga")
            .spawn()?;

        Ok(())
    }

    pub fn next(&self, now: &DateTime<Local>) -> Result<DateTime<Local>> {
        let naive_time = NaiveTime::from_hms_opt(self.hour.into(), self.minute.into(), 0).ok_or(
            Error::InvalidTime(self.hour.to_string(), self.minute.to_string()),
        )?;

        let mut run_today = match now.with_time(naive_time) {
            chrono::offset::LocalResult::Single(date) => Ok(date),
            chrono::offset::LocalResult::Ambiguous(_, _) => Err(Error::AmbiguousTime),
            chrono::offset::LocalResult::None => Err(Error::NoLocalTime),
        }?;

        loop {
            if &run_today >= now {
                let day = match run_today.weekday() {
                    chrono::Weekday::Mon => day::MONDAY,
                    chrono::Weekday::Tue => day::TUESDAY,
                    chrono::Weekday::Wed => day::WEDNESDAY,
                    chrono::Weekday::Thu => day::THURSDAY,
                    chrono::Weekday::Fri => day::FRIDAY,
                    chrono::Weekday::Sat => day::SATURDAY,
                    chrono::Weekday::Sun => day::SUNDAY,
                };

                if self.day & day == day {
                    break;
                }
            }

            run_today += Duration::days(1);
        }

        Ok(run_today)
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
}

#[test]
fn test_next() -> Result<()> {
    let n = Notification::new(
        1,
        "Title".to_string(),
        "Mesage".to_string(),
        "10:00".to_string(),
        day::MONDAY | day::WEDNESDAY,
    )?;

    let now = create_date(2025, 1, 1, 9, 30)?;
    assert_eq!(create_date(2025, 1, 1, 10, 0)?, n.next(&now)?);

    let now = create_date(2025, 1, 1, 10, 0)?;
    assert_eq!(create_date(2025, 1, 1, 10, 0)?, n.next(&now)?);

    let now = create_date(2025, 1, 3, 10, 30)?;
    assert_eq!(create_date(2025, 1, 6, 10, 0)?, n.next(&now)?);

    let now = create_date(2025, 1, 6, 10, 0)?;
    assert_eq!(create_date(2025, 1, 6, 10, 0)?, n.next(&now)?);

    let now = create_date(2025, 1, 6, 10, 1)?;
    assert_eq!(create_date(2025, 1, 8, 10, 0)?, n.next(&now)?);

    let now = create_date(2025, 1, 7, 9, 0)?;
    assert_eq!(create_date(2025, 1, 8, 10, 0)?, n.next(&now)?);

    Ok(())
}

#[cfg(test)]
fn create_date(year: i32, month: u32, day: u32, hour: u32, min: u32) -> Result<DateTime<Local>> {
    use chrono::TimeZone;

    Local
        .with_ymd_and_hms(year, month, day, hour, min, 0)
        .single()
        .ok_or(Error::String("Empty".to_string()))
}
