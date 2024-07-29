use crate::{config, switch};
use clap::Parser;
use color_eyre::eyre::{eyre, Result};
use url::Url;

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

        if let Ok(url) = Url::parse(&userchrome.source) {
            if url.scheme() == "http" || url.scheme() == "https" {
                switch::switch(userchrome, &config.profile).await?;
                println!("Refreshed userchrome: {}", self.name);
            } else {
                println!("Skipping refresh: {} is not a remote source", self.name);
            }
        } else {
            println!("Skipping refresh: {} is not a valid URL", self.name);
        }

        Ok(())
    }
}
