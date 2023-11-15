#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![deny(unsafe_code)]

use anyhow::Result;
use clap::Parser;
use cmd::{Cli, Command};

mod cmd;
mod config;
mod presets;
mod switch;
mod utils;

use owo_colors::OwoColorize;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.command.action(&cli).await?;
    Ok(())
}
