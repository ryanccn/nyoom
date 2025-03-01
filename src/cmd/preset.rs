// SPDX-FileCopyrightText: 2024 Ryan Cao <hello@ryanccn.dev>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use clap::Parser;
use eyre::{bail, eyre, Result};

use crate::{config, presets::PRESETS};

#[derive(Parser)]
pub struct PresetCommand {
    /// Name of the preset to add
    name: Option<String>,
}

impl super::Command for PresetCommand {
    async fn action(&self, global_options: &super::Cli) -> Result<()> {
        if let Some(name) = &self.name {
            let preset = PRESETS
                .iter()
                .find(|p| &p.name == name)
                .ok_or_else(|| eyre!("no preset named {:?} exists!", name))?;

            let mut config = config::get_config(&global_options.config).await?;

            if config.userchromes.iter().any(|uc| uc.name == preset.name) {
                bail!("the userchrome {:?} already exists!", self.name);
            }

            config::print_userchrome(preset, false, &config::PrintContext::Added);
            config.userchromes.push(preset.to_owned());

            config::set_config(&global_options.config, &config).await?;
        } else {
            for p in PRESETS.iter() {
                config::print_userchrome(p, true, &config::PrintContext::Normal);
            }
        }

        Ok(())
    }
}
