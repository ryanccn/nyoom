// SPDX-FileCopyrightText: 2024 Ryan Cao <hello@ryanccn.dev>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use clap::Parser;
use eyre::Result;

use crate::config;

#[derive(Parser)]
pub struct ListCommand {}

impl super::Command for ListCommand {
    async fn action(&self, global_options: &super::Cli) -> Result<()> {
        let config = config::get_config(&global_options.config).await?;
        for u in config.userchromes {
            config::print_userchrome(&u, false, &config::PrintContext::Normal);
        }

        Ok(())
    }
}
