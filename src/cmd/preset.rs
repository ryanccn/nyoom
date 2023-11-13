use anyhow::{anyhow, Result};
use async_trait::async_trait;
use clap::Parser;

use crate::{config, presets};

#[derive(Parser)]
pub struct PresetCommand {
    /// Name of the preset
    name: Option<String>,
}

#[async_trait]
impl super::Command for PresetCommand {
    async fn action(&self, global_options: &super::Cli) -> Result<()> {
        let presets = presets::get_presets()?;

        if let Some(name) = &self.name {
            let preset = presets
                .into_iter()
                .find(|p| p.name == *name)
                .ok_or(anyhow!("no preset named {} exists!", name))?;

            let mut config = config::get_config(&global_options.config).await?;

            config::print_userchrome(&preset, false);
            config.userchromes.push(preset);

            config::set_config(&global_options.config, &config).await?;
        } else {
            for p in presets {
                config::print_userchrome(&p, true);
            }
        }

        Ok(())
    }
}
