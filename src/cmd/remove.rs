// SPDX-FileCopyrightText: 2024 Ryan Cao <hello@ryanccn.dev>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use clap::Parser;
use eyre::{Result, eyre};

use crate::config;

#[derive(Parser)]
pub struct RemoveCommand {
    /// Name of the userchrome to remove
    name: String,
}

impl super::Command for RemoveCommand {
    async fn action(&self, global_options: &super::Cli) -> Result<()> {
        let mut config = config::get_config(&global_options.config).await?;

        let res = config
            .userchromes
            .iter()
            .enumerate()
            .find(|(_, uchrome)| uchrome.name == self.name);

        if let Some((i, uchrome)) = res {
            config::print_userchrome(uchrome, true, &config::PrintContext::Removed);

            config.userchromes.remove(i);
            config::set_config(&global_options.config, &config).await?;
            Ok(())
        } else {
            Err(eyre!(
                "no userchrome with name {:?} found to remove!",
                self.name
            ))
        }
    }
}
