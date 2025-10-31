#![allow(clippy::needless_range_loop)]
use crate::error::Result;
use clap::Parser;
use command::Commands;
use notification::Notification;

mod command;
mod database;
mod day;
mod error;
mod notification;
mod schedule;

#[derive(Parser)]
#[command(name = "herd", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Serve => command::serve(),
        Commands::List => command::list(),
        Commands::Add {
            title,
            message,
            time,
            sunday,
            monday,
            tuesday,
            wednesday,
            thursday,
            friday,
            saturday,
            weekday,
            weekend,
        } => command::add(
            title, message, time, sunday, monday, tuesday, wednesday, thursday, friday, saturday,
            weekday, weekend,
        ),
        Commands::Remove => todo!(),
        Commands::Upgrade => todo!(),
    };

    if let Err(result) = result {
        Notification::error_now(result.to_string())?;
    }

    Ok(())
}
