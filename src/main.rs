use anyhow::Result;
use clap::Parser;
use command::Commands;

mod command;
mod database;

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
        Commands::List => todo!(),
        Commands::Add => todo!(),
        Commands::Remove => todo!(),
        Commands::Upgrade => todo!(),
    }
}
