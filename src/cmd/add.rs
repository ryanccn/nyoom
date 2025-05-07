// SPDX-FileCopyrightText: 2024 Ryan Cao <hello@ryanccn.dev>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use clap::Parser;
use eyre::{Result, bail};

use crate::{config, source::ParsedSource};

#[derive(Parser)]
pub struct AddCommand {
    /// Name of the userchrome
    name: String,
    /// Source specification
    source: String,
}

impl super::Command for AddCommand {
    async fn action(&self, global_options: &super::Cli) -> Result<()> {
        let mut config = config::Config::read(&global_options.config).await?;

        if config.userchromes.iter().any(|c| c.name == self.name) {
            bail!("the userchrome {:?} already exists!", self.name);
        }

        let parsed: ParsedSource = self.source.parse()?;

        let new_userchrome = config::Userchrome {
            name: self.name.clone(),
            source: if parsed.should_canonicalize() {
                parsed.to_string()
            } else {
                self.source.clone()
            },
            configs: Vec::new(),
        };

        new_userchrome.print(false, config::PrintContext::Added);
        config.userchromes.push(new_userchrome);

        config.write(&global_options.config).await?;

        Ok(())
    }
}
