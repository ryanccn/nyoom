// SPDX-FileCopyrightText: 2024 Ryan Cao <hello@ryanccn.dev>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::path::PathBuf;

use clap::{Parser, ValueHint};
use eyre::{bail, Result};
use owo_colors::OwoColorize as _;

use crate::config;

#[derive(Parser)]
pub struct ProfileCommand {
    /// Path to the profile
    #[arg(value_hint = ValueHint::DirPath)]
    path: Option<PathBuf>,
}

impl super::Command for ProfileCommand {
    async fn action(&self, global_options: &super::Cli) -> Result<()> {
        if let Some(path) = &self.path {
            if !path.is_dir() {
                bail!("profile does not exist");
            }

            let mut config = config::get_config(&global_options.config).await?;
            config.profile = Some(path.clone());
            config::set_config(&global_options.config, &config).await?;
        } else {
            let config = config::get_config(&global_options.config).await?;
            println!(
                "{}",
                config.profile.map_or_else(
                    || "not set".red().to_string(),
                    |profile| profile.display().to_string()
                )
            );
        }

        Ok(())
    }
}
