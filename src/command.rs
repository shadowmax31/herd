use anyhow::Result;
use clap::Subcommand;

use crate::database::create_connection;

#[derive(Subcommand)]
pub enum Commands {
    Serve,
    List,
    Add,
    Remove,
    Upgrade,
}

pub fn serve() -> Result<()> {
    let connection = create_connection()?;

    Ok(())
}
