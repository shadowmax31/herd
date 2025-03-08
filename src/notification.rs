use anyhow::Result;
use chrono::{DateTime, Duration, Local};
use rusqlite::Connection;

use crate::day;

#[derive(Debug)]
pub struct Notification {
    id: usize,
    title: String,
    message: String,
    time: String,
    day: u8,
}

pub trait Task {
    fn run(&self) -> Result<()>;
    fn next(&self) -> DateTime<Local>;
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

        Ok(())
    }

    fn next(&self) -> DateTime<Local> {
        Local::now() + Duration::seconds(5)
    }
}

impl Notification {
    pub fn new(id: usize, title: String, message: String, time: String, day: u8) -> Self {
        Notification {
            id,
            title,
            message,
            time,
            day,
        }
    }

    pub fn simple_print(&self) -> String {
        format!(
            "{}: {} | Runs on `{}` at {}",
            self.id,
            self.title,
            day::to_string(self.day),
            self.time,
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
            list.push(notification?);
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
