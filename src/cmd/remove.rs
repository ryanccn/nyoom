use async_trait::async_trait;
use clap::Parser;
use color_eyre::eyre::{eyre, Result};

use crate::config;

#[derive(Parser)]
pub struct RemoveCommand {
    /// Name of the userchrome
    name: String,
}

#[async_trait]
impl super::Command for RemoveCommand {
    async fn action(&self, global_options: &super::Cli) -> Result<()> {
        let mut config = config::get_config(&global_options.config).await?;

        let res = config
            .userchromes
            .iter()
            .enumerate()
            .find(|(_i, uchrome)| uchrome.name.eq(&self.name));

        match res {
            Some((i, uchrome)) => {
                println!("Removing!");

                config::print_userchrome(&uchrome, false);

                config.userchromes.remove(i);

                config::set_config(&global_options.config, &config).await?;
                return Ok(());
            }
            None => {
                return Err(eyre!(
                    "no userchrome with name {} found to remove!",
                    self.name
                ));
            }
        };
    }
}
