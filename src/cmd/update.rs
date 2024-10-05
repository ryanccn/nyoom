// SPDX-FileCopyrightText: 2024 Ryan Cao <hello@ryanccn.dev>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::fs;

use clap::Parser;
use eyre::{bail, Result};

use crate::{config, switch, utils};

#[derive(Parser)]
pub struct UpdateCommand {}

impl super::Command for UpdateCommand {
    async fn action(&self, global_options: &super::Cli) -> Result<()> {
        let config = config::get_config(&global_options.config).await?;

        utils::check_firefox()?;

        if let Some(profile) = &config.profile {
            let current = fs::read_to_string(profile.join("chrome").join(".nyoom-chrome-name"))
                .map(|s| s.trim().to_owned())
                .ok();

            if let Some(u) = config
                .userchromes
                .iter()
                .find(|c| Some(&c.name) == current.as_ref())
            {
                switch::switch(Some(u), profile).await?;
            } else {
                bail!("no installed userchrome found!");
            }
        } else {
            bail!("no profile configured");
        }

        Ok(())
    }
}
