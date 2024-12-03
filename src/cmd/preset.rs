// SPDX-FileCopyrightText: 2024 Ryan Cao <hello@ryanccn.dev>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use clap::Parser;
use eyre::{bail, eyre, Result};

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
                .ok_or_else(|| eyre!("no preset named {:?} exists!", name))?;

            let mut config = config::get_config(&global_options.config).await?;

            if config.userchromes.iter().any(|uc| uc.name == preset.name) {
                bail!("the userchrome {:?} already exists!", self.name);
            }

            config::print_userchrome(&preset, false, &config::PrintContext::Added);
            config.userchromes.push(preset);

            config::set_config(&global_options.config, &config).await?;
        } else {
            for p in presets {
                config::print_userchrome(&p, true, &config::PrintContext::Normal);
            }
        }

        Ok(())
    }
}
