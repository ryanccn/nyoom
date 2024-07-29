use crate::{config, switch};
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

        let profile = &config.profile;

        // Use the existing switch function to refresh the userchrome
        switch::switch(userchrome, profile.to_string()).await?;

        println!("Refreshed userchrome: {}", self.name);
        Ok(())
    }
}
