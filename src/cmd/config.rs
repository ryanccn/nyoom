// SPDX-FileCopyrightText: 2024 Ryan Cao <hello@ryanccn.dev>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use anstream::println;
use clap::{Parser, Subcommand};
use eyre::{eyre, Result};

use crate::config;

#[derive(Parser)]
pub struct ConfigCommand {
    #[command(subcommand)]
    command: ConfigSubcommands,
}

#[derive(Subcommand)]
enum ConfigSubcommands {
    /// List Firefox configs
    List { name: String },

    /// Set a Firefox config
    Set {
        /// Name of the userchrome
        name: String,
        /// Config key
        key: String,
        /// Config value
        value: String,

        #[arg(short, long)]
        /// Whether the value is a raw value or a string
        raw: bool,
    },

    /// Unset a Firefox config
    Unset {
        /// Name of the userchrome
        name: String,
        /// Config key
        key: String,
    },
}

impl super::Command for ConfigCommand {
    async fn action(&self, global_options: &super::Cli) -> Result<()> {
        match &self.command {
            ConfigSubcommands::List { name } => {
                let config = config::get_config(&global_options.config).await?;
                let uc = config
                    .userchromes
                    .iter()
                    .find(|d| &d.name == name)
                    .ok_or_else(|| eyre!("no userchrome with name {:?} exists", name))?;

                for c in &uc.configs {
                    println!("{}", config::format_userchrome_config(c));
                }

                Ok(())
            }

            ConfigSubcommands::Set {
                name,
                key,
                value,
                raw,
            } => {
                let mut config = config::get_config(&global_options.config).await?;
                let chrome = config
                    .userchromes
                    .iter_mut()
                    .find(|d| &d.name == name)
                    .ok_or_else(|| eyre!("no userchrome with name {:?} exists", name))?;

                let existing = chrome.configs.iter_mut().find(|c| c.key == *key);

                if let Some(existing) = existing {
                    existing.value.clone_from(value);
                    existing.raw.clone_from(raw);
                } else {
                    chrome.configs.push(config::UserchromeConfig {
                        key: key.clone(),
                        value: value.clone(),
                        raw: *raw,
                    });
                }

                config::set_config(&global_options.config, &config).await?;

                Ok(())
            }

            ConfigSubcommands::Unset { name, key } => {
                let mut config = config::get_config(&global_options.config).await?;
                let chrome = config
                    .userchromes
                    .iter_mut()
                    .find(|d| &d.name == name)
                    .ok_or_else(|| eyre!("no userchrome with name {:?} exists", name))?;

                chrome.configs.retain(|c| c.key != *key);

                config::set_config(&global_options.config, &config).await?;

                Ok(())
            }
        }
    }
}
