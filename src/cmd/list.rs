use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;

use crate::config;

#[derive(Parser)]
pub struct ListCommand {}

#[async_trait]
impl super::Command for ListCommand {
    async fn action(&self, global_options: &super::Cli) -> Result<()> {
        let config = config::get_config(&global_options.config).await?;
        for u in config.userchromes {
            config::print_userchrome(&u, false);
        }

        Ok(())
    }
}
