use clap::Parser;
use color_eyre::eyre::{bail, Result};

use crate::{config, utils};

#[derive(Parser)]
pub struct AddCommand {
    /// Name of the userchrome
    name: String,
    /// Git clone URL
    source: String,
}

impl super::Command for AddCommand {
    async fn action(&self, global_options: &super::Cli) -> Result<()> {
        let mut config = config::get_config(&global_options.config).await?;

        if config.userchromes.iter().any(|uc| uc.name == self.name) {
            bail!("The userchrome {} already exists!", self.name);
        }

        let cache_path = config.cache_dir.join(&self.name);

        if utils::is_remote_source(&self.source) {
            utils::download_and_cache(&self.source, &cache_path).await?;
        }

        let new_userchrome = config::Userchrome {
            name: self.name.clone(),
            source: self.source.clone(),
            configs: vec![],
            clone_url: None,
            cache_path: Some(cache_path),
        };

        config::print_userchrome(&new_userchrome, false);
        config.userchromes.push(new_userchrome);

        config::set_config(&global_options.config, &config).await?;

        Ok(())
    }
}
