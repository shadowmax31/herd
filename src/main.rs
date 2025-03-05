use clap::{Parser, Subcommand};

#[derive(Subcommand)]
enum Commands {
    Serve,
    List,
    Add,
    Remove,
}

#[derive(Parser)]
#[command(name = "herd", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let cli = Cli::parse();
}
