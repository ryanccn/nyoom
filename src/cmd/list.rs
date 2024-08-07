use clap::Parser;
use color_eyre::eyre::Result;

use crate::config;

#[derive(Parser)]
pub struct ListCommand {}

impl super::Command for ListCommand {
    async fn action(&self, global_options: &super::Cli) -> Result<()> {
        let config = config::get_config(&global_options.config).await?;
        for u in config.userchromes {
            config::print_userchrome(&u, false);
        }

        Ok(())
    }
}
