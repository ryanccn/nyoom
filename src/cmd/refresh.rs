use crate::{config, switch, utils};
use clap::Parser;
use color_eyre::eyre::{eyre, Result};

#[derive(Parser)]
pub struct RefreshCommand {
    /// Name of the userchrome to refresh
    name: String,
}

impl super::Command for RefreshCommand {
    async fn action(&self, global_options: &super::Cli) -> Result<()> {
        let config = config::get_config(&global_options.config).await?;
        let userchrome = config
            .userchromes
            .iter()
            .find(|uc| uc.name == self.name)
            .ok_or_else(|| eyre!("Userchrome {} not found", self.name))?;

        let cache_path = userchrome
            .cache_path
            .as_ref()
            .ok_or_else(|| eyre!("Cache path not found for userchrome {}", self.name))?;

        if utils::is_remote_source(&userchrome.source) {
            let updated = utils::download_and_cache(&userchrome.source, cache_path).await?;
            if updated {
                switch::switch(userchrome, config.profile.clone()).await?;
                println!("Refreshed userchrome: {}", self.name);
            } else {
                println!("Userchrome {} is already up to date", self.name);
            }
        } else {
            println!("Skipping refresh: {} is not a remote source", self.name);
        }

        Ok(())
    }
}
