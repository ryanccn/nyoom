// SPDX-FileCopyrightText: 2024 Ryan Cao <hello@ryanccn.dev>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use clap::Parser;
use eyre::{eyre, Result};

use crate::{config, presets};

#[derive(Parser)]
pub struct PresetCommand {
    /// Name of the preset
    name: Option<String>,
}

impl super::Command for PresetCommand {
    async fn action(&self, global_options: &super::Cli) -> Result<()> {
        let presets = presets::get_presets()?;

        if let Some(name) = &self.name {
            let preset = presets
                .into_iter()
                .find(|p| &p.name == name)
                .ok_or_else(|| eyre!("no preset named {} exists!", name))?;

            let mut config = config::get_config(&global_options.config).await?;

            config::print_userchrome(&preset, false);
            config.userchromes.push(preset);

            config::set_config(&global_options.config, &config).await?;
        } else {
            for p in presets {
                config::print_userchrome(&p, true);
            }
        }

        Ok(())
    }
}
