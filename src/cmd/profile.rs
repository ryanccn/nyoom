use clap::{Parser, ValueHint};
use color_eyre::eyre::Result;

use crate::config;

#[derive(Parser)]
pub struct ProfileCommand {
    /// Path to the profile
    #[arg(value_hint = ValueHint::DirPath)]
    path: Option<String>,
}

impl super::Command for ProfileCommand {
    async fn action(&self, global_options: &super::Cli) -> Result<()> {
        if let Some(path) = &self.path {
            let mut config = config::get_config(&global_options.config).await?;
            config.profile.clone_from(path);
            config::set_config(&global_options.config, &config).await?;
        } else {
            let config = config::get_config(&global_options.config).await?;
            println!(
                "{}",
                if config.profile.is_empty() {
                    "not set".into()
                } else {
                    config.profile
                }
            );
        }

        Ok(())
    }
}
