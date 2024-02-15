use async_trait::async_trait;
use clap::Parser;
use color_eyre::eyre::Result;

use crate::config;

#[derive(Parser)]
pub struct AddCommand {
    /// Name of the userchrome
    name: String,
    /// Git clone URL
    source: String,
}

#[async_trait]
impl super::Command for AddCommand {
    async fn action(&self, global_options: &super::Cli) -> Result<()> {
        let mut config = config::get_config(&global_options.config).await?;

        let new_userchrome = config::Userchrome {
            name: self.name.clone(),
            source: self.source.clone(),
            clone_url: None,
            configs: vec![],
        };
        config::print_userchrome(&new_userchrome, false);
        config.userchromes.push(new_userchrome);

        config::set_config(&global_options.config, &config).await?;

        Ok(())
    }
}
