use anyhow::{anyhow, Result};
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

pub trait Task {
    fn run(&self) -> Result<()>;
    fn next(&self, now: &DateTime<Local>) -> DateTime<Local>;
}

impl Task for Notification {
    fn run(&self) -> Result<()> {
        let now = chrono::offset::Local::now();

        std::process::Command::new("notify-send")
            .arg(&self.title)
            .arg(format!(
                "{} \n\n{}",
                &self.message,
                now.format("%Y-%m-%d %H:%M:%S"),
            ))
            .spawn()?;

        std::process::Command::new("play")
            .arg("-q")
            .arg("/usr/share/sounds/freedesktop/stereo/bell.oga")
            .spawn()?;

        Ok(())
    }

    fn next(&self, now: &DateTime<Local>) -> DateTime<Local> {
        let run_today = now
            .with_time(NaiveTime::from_hms_opt(self.hour.into(), self.minute.into(), 0).unwrap())
            .unwrap();

        let mut run_today: DateTime<Local> = run_today;

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

            run_today = run_today + Duration::days(1);
        }

        run_today
    }
}

impl Notification {
    pub fn new(id: usize, title: String, message: String, time: String, day: u8) -> Result<Self> {
        let hour_minute: Vec<&str> = time.split(":").collect();

        let time_parse_error = format!("Could not parse time {}", time);
        let hour: u8 = hour_minute
            .get(0)
            .ok_or(anyhow!(time_parse_error.clone()))?
            .parse()?;
        let minute: u8 = hour_minute
            .get(1)
            .ok_or(anyhow!(time_parse_error))?
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

    pub fn simple_print(&self) -> String {
        format!(
            "{}: {} | Runs on `{}` at {}:{}",
            self.id,
            self.title,
            day::to_string(self.day),
            self.hour,
            self.minute,
        )
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
    assert_eq!(create_date(2025, 1, 1, 10, 0)?, n.next(&now));

    let now = create_date(2025, 1, 1, 10, 0)?;
    assert_eq!(create_date(2025, 1, 1, 10, 0)?, n.next(&now));

    let now = create_date(2025, 1, 3, 10, 30)?;
    assert_eq!(create_date(2025, 1, 6, 10, 0)?, n.next(&now));

    let now = create_date(2025, 1, 6, 10, 0)?;
    assert_eq!(create_date(2025, 1, 6, 10, 0)?, n.next(&now));

    let now = create_date(2025, 1, 6, 10, 1)?;
    assert_eq!(create_date(2025, 1, 8, 10, 0)?, n.next(&now));

    let now = create_date(2025, 1, 7, 9, 0)?;
    assert_eq!(create_date(2025, 1, 8, 10, 0)?, n.next(&now));

    Ok(())
}

#[cfg(test)]
fn create_date(year: i32, month: u32, day: u32, hour: u32, min: u32) -> Result<DateTime<Local>> {
    use chrono::TimeZone;

    Local
        .with_ymd_and_hms(year, month, day, hour, min, 0)
        .single()
        .ok_or(anyhow!("Empty"))
}
