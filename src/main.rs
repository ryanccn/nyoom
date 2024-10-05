// SPDX-FileCopyrightText: 2024 Ryan Cao <hello@ryanccn.dev>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use clap::Parser;
use cmd::{Cli, Command};
use config::migrate_config;
use eyre::Result;

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
