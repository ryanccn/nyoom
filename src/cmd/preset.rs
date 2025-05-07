// SPDX-FileCopyrightText: 2024 Ryan Cao <hello@ryanccn.dev>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use clap::Parser;
use eyre::{Result, bail, eyre};

use crate::{config, presets::PRESETS};

#[derive(Parser)]
pub struct PresetCommand {
    /// Name of the preset to add
    name: Option<String>,
}

impl super::Command for PresetCommand {
    async fn action(&self, global_options: &super::Cli) -> Result<()> {
        if let Some(name) = &self.name {
            let mut config = config::Config::read(&global_options.config).await?;

            if config.userchromes.iter().any(|c| &c.name == name) {
                bail!("the userchrome {:?} already exists!", self.name);
            }

            let preset = PRESETS
                .iter()
                .find(|p| &p.name == name)
                .ok_or_else(|| eyre!("no preset named {:?} exists!", name))?;

            preset.print(false, config::PrintContext::Added);
            config.userchromes.push(preset.to_owned());

            config.write(&global_options.config).await?;
        } else {
            for p in PRESETS.iter() {
                p.print(true, config::PrintContext::Normal);
            }
        }

        Ok(())
    }
}
