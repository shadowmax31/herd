use anyhow::Result;
use clap::Parser;
use command::Commands;

mod command;
mod database;
mod day;
mod entity;

#[derive(Parser)]
#[command(name = "herd", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
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
    }
}
