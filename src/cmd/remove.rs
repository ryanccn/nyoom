use clap::Parser;
use color_eyre::{
    eyre::{eyre, Result},
    owo_colors::OwoColorize as _,
};
use tokio::fs;

use crate::config;

#[derive(Parser)]
pub struct RemoveCommand {
    /// Name of the userchrome
    name: String,
}

impl super::Command for RemoveCommand {
    async fn action(&self, global_options: &super::Cli) -> Result<()> {
        let mut config = config::get_config(&global_options.config).await?;

        let res = config
            .userchromes
            .iter()
            .enumerate()
            .find(|(_, uchrome)| uchrome.name.eq(&self.name));

        match res {
            Some((i, uchrome)) => {
                println!("Removing {}!", uchrome.name.cyan());
                config::print_userchrome(uchrome, true);

                // Remove cache if it exists
                if let Some(cache_path) = &uchrome.cache_path {
                    if cache_path.exists() {
                        fs::remove_dir_all(cache_path).await?;
                    }
                }

                config.userchromes.remove(i);
                config::set_config(&global_options.config, &config).await?;
                Ok(())
            }
            None => Err(eyre!(
                "No userchrome with name {} found to remove!",
                self.name
            )),
        }
    }
}
