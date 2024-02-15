#![warn(clippy::all, clippy::pedantic, clippy::perf)]
#![allow(clippy::module_name_repetitions)]
#![forbid(unsafe_code)]

use clap::Parser;
use cmd::{Cli, Command};
use color_eyre::eyre::Result;
use config::migrate_config;

mod cmd;
mod config;
mod presets;
mod switch;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    migrate_config().await?;
    let cli = Cli::parse();
    cli.command.action(&cli).await?;

    Ok(())
}
