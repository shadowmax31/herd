use anyhow::Result;
use chrono::{DateTime, Local};
use rusqlite::Connection;

use crate::{
    database::{close_connection, create_connection},
    notification::Notification,
};

pub struct Schedule {
    items: Vec<ScheduleItem>,
}

struct ScheduleItem {
    next: DateTime<Local>,
    notification: Notification,
}

impl Schedule {
    pub fn initial_schedule() -> Result<Schedule> {
        let connection = create_connection()?;

        let mut items = vec![];
        for n in Notification::find_all(&connection)? {
            items.push(ScheduleItem::new(n, &Local::now()));
        }

        close_connection(connection)?;

        Ok(Schedule { items })
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    fn update(&mut self, connection: &Connection, now: &DateTime<Local>) -> Result<()> {
        let ids: Vec<usize> = self.items.iter().map(|x| x.get_id()).collect();
        for n in Notification::find_all(connection)? {
            if !ids.contains(&n.get_id()) {
                let item = ScheduleItem::new(n, now);
                Notification::notify_now(format!(
                    "Added `{}`, will run on {}",
                    item.notification.get_title(),
                    &item.next.format("%Y-%m-%d %H:%M")
                ))?;
                self.items.push(item);
            }
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        let now = Local::now();

        let connection = create_connection()?;
        self.update(&connection, &now)?;
        close_connection(connection)?;

        for t in &mut self.items {
            t.run(&now)?;
        }

        Ok(())
    }
}

impl ScheduleItem {
    fn new(notification: Notification, now: &DateTime<Local>) -> ScheduleItem {
        ScheduleItem {
            next: notification.next(now),
            notification,
        }
    }

    fn run(&mut self, now: &DateTime<Local>) -> Result<()> {
        if &self.next < now {
            self.notification.run(false)?;
            self.next = self.notification.next(now);
        }
        Ok(())
    }

    fn get_id(&self) -> usize {
        self.notification.get_id()
    }
}
