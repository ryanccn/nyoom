use std::path::PathBuf;
use tokio::fs;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use clap::Parser;

use crate::{config, switch, utils};

#[derive(Parser)]
pub struct SwitchCommand {
    /// Name of the userchrome
    name: String,
}

#[async_trait]
impl super::Command for SwitchCommand {
    async fn action(&self, global_options: &super::Cli) -> Result<()> {
        let config = config::get_config(&global_options.config).await?;

        utils::check_firefox();

        if self.name == "out" {
            let profile_path = config.profile.parse::<PathBuf>()?;
            fs::remove_dir_all(profile_path.join("chrome")).await?;
        } else {
            match config.userchromes.iter().find(|c| c.name.eq(&self.name)) {
                Some(u) => {
                    if config.profile.is_empty() {
                        return Err(anyhow!("no profile configured"));
                    }

                    switch::switch(u, config.profile).await?;
                }
                None => {
                    return Err(anyhow!("no userchrome with name {} found!", self.name));
                }
            };
        };

        Ok(())
    }
}
