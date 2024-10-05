// SPDX-FileCopyrightText: 2024 Ryan Cao <hello@ryanccn.dev>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use clap::Parser;
use eyre::{bail, Result};

use crate::config;

#[derive(Parser)]
pub struct AddCommand {
    /// Name of the userchrome
    name: String,
    /// Git clone URL
    source: String,
}

impl super::Command for AddCommand {
    async fn action(&self, global_options: &super::Cli) -> Result<()> {
        let mut config = config::get_config(&global_options.config).await?;

        if config.userchromes.iter().any(|uc| uc.name == self.name) {
            bail!("the userchrome {} already exists!", self.name);
        }

        let new_userchrome = config::Userchrome {
            name: self.name.clone(),
            source: self.source.clone(),
            clone_url: None,
            configs: Vec::new(),
        };

        config::print_userchrome(&new_userchrome, false);
        config.userchromes.push(new_userchrome);

        config::set_config(&global_options.config, &config).await?;

        Ok(())
    }
}
