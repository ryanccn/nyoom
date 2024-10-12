// SPDX-FileCopyrightText: 2024 Ryan Cao <hello@ryanccn.dev>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use clap::Parser;
use eyre::{bail, Result};

use crate::{config, switch, utils};

#[derive(Parser)]
pub struct SwitchCommand {
    /// Name of the userchrome
    name: String,
}

impl super::Command for SwitchCommand {
    async fn action(&self, global_options: &super::Cli) -> Result<()> {
        let config = config::get_config(&global_options.config).await?;

        utils::check_firefox();

        if let Some(profile) = &config.profile {
            if self.name == "out" {
                switch::switch(None, profile).await?;
            } else if let Some(u) = config.userchromes.iter().find(|c| c.name == self.name) {
                switch::switch(Some(u), profile).await?;
            } else {
                bail!("no userchrome with name {} found!", self.name);
            };
        } else {
            bail!("no profile configured");
        }

        Ok(())
    }
}
