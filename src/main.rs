#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![deny(unsafe_code)]

use anyhow::Result;
use clap::Parser;
use cmd::{Cli, Command};
use config::migrate_config;

mod cmd;
mod config;
mod presets;
mod switch;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    migrate_config().await?;
    let cli = Cli::parse();
    cli.command.action(&cli).await?;
    Ok(())
}
