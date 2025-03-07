use anyhow::Result;
use chrono::{DateTime, Local};

use crate::notification::Task;

pub struct Schedule<'a> {
    next: DateTime<Local>,
    task: &'a dyn Task,
}

impl<'a> Schedule<'a> {
    pub fn new(task: &'a impl Task) -> Schedule<'a> {
        Schedule {
            next: task.next(),
            task,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let now = Local::now();
        if self.next < now {
            self.task.run()?;
            self.next = self.task.next();
        }
        Ok(())
    }
}
