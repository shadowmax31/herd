use std::time::Duration;

use anyhow::Result;
use chrono::Local;
use clap::Subcommand;

use crate::{database::create_connection, day, notification::Notification, schedule::Schedule};

#[derive(Subcommand)]
pub enum Commands {
    Serve,
    List,
    Add {
        title: String,
        message: String,
        time: String,

        #[arg(long)]
        sunday: bool,
        #[arg(long)]
        monday: bool,
        #[arg(long)]
        tuesday: bool,
        #[arg(long)]
        wednesday: bool,
        #[arg(long)]
        thursday: bool,
        #[arg(long)]
        friday: bool,
        #[arg(long)]
        saturday: bool,
        #[arg(long)]
        weekday: bool,
        #[arg(long)]
        weekend: bool,
    },
    Remove,
    Upgrade,
}

pub fn serve() -> Result<()> {
    let connection = create_connection()?;

    let mut tasks = vec![];
    let notifications = Notification::find_all(&connection)?;
    for n in &notifications {
        tasks.push(Schedule::new(n, &Local::now()));
    }

    loop {
        let now = Local::now();
        for t in &mut tasks {
            t.run(&now)?;
        }

        std::thread::sleep(Duration::from_millis(100));
    }
}

pub fn list() -> Result<()> {
    let connection = create_connection()?;

    for n in Notification::find_all(&connection)? {
        println!("{}", n.simple_print());
    }

    Ok(())
}

pub fn add(
    title: String,
    message: String,
    time: String,
    sunday: bool,
    monday: bool,
    tuesday: bool,
    wednesday: bool,
    thursday: bool,
    friday: bool,
    saturday: bool,
    weekday: bool,
    weekend: bool,
) -> Result<()> {
    let day = day::to_day(
        sunday, monday, tuesday, wednesday, thursday, friday, saturday, weekday, weekend,
    );

    let connection = create_connection()?;
    Notification::insert(&connection, title, message, time, day)?;

    Ok(())
}
