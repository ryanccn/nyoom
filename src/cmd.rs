use clap::{CommandFactory, Parser, Subcommand, ValueHint};
use clap_complete::{generate, Shell};

use anyhow::{anyhow, Result};
use std::io;

use crate::{config, presets, switch};

#[derive(Parser)]
#[command(author, version, about = "\x1B[36;1mnyoom · Firefox userchrome manager\x1B[0m", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Config file to use
    #[arg(short, long, default_value_t = config::get_default_config_path().unwrap().display().to_string().into(), value_hint = ValueHint::FilePath)]
    config: String,
}

#[derive(Subcommand)]
enum Commands {
    /// List userchromes
    List {},

    /// Add a new userchrome
    Add {
        /// Name of the userchrome
        name: String,
        /// Git clone URL
        source: String,
    },

    /// Switch to a userchrome
    Switch {
        /// Name of the userchrome
        name: String,
    },

    /// Import a preset as a userchrome or list presets
    Preset {
        /// Name of the preset
        name: Option<String>,
    },

    /// Configure Firefox profile or get current directory
    Profile {
        /// Path to the profile
        #[arg(value_hint = ValueHint::DirPath)]
        path: Option<String>,
    },

    /// Manage userchrome-linked configs
    Config {
        #[command(subcommand)]
        command: ConfigSubcommands,
    },

    /// Generate completions
    Completions {
        /// Shell
        #[arg(value_enum)]
        shell: Shell,
    },
}

#[derive(Subcommand)]
enum ConfigSubcommands {
    /// List Firefox configs
    List { name: String },

    /// Set a Firefox config
    Set {
        /// Name of the userchrome
        name: String,
        /// Config key
        key: String,
        /// Config value
        value: String,

        #[arg(short, long)]
        /// Whether the value is raw or a string
        raw: bool,
    },

    /// Unset a Firefox config
    Unset {
        /// Name of the userchrome
        name: String,
        /// Config key
        key: String,
    },
}

pub fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::List {} => {
            let config = config::get_config(&cli.config)?;
            for u in config.userchromes {
                config::print_userchrome(&u, false);
            }

            Ok(())
        }

        Commands::Add { name, source } => {
            let mut config = config::get_config(&cli.config)?;

            let new_userchrome = config::Userchrome {
                name: name.into(),
                source: source.into(),
                clone_url: None,
                configs: vec![],
            };
            config::print_userchrome(&new_userchrome, false);
            config.userchromes.push(new_userchrome);

            config::set_config(&cli.config, &config)?;

            Ok(())
        }

        Commands::Switch { name } => {
            let config = config::get_config(&cli.config)?;
            match config.userchromes.iter().find(|c| c.name.eq(name)) {
                Some(u) => {
                    if config.profile.is_empty() {
                        return Err(anyhow!("no profile configured"));
                    }

                    switch::switch(u, config.profile)?;
                }
                None => {
                    return Err(anyhow!("no userchrome with name {} found!", name));
                }
            };

            Ok(())
        }

        Commands::Preset { name } => {
            let presets = presets::get_presets()?;

            if let Some(name) = name {
                let preset = presets
                    .into_iter()
                    .find(|p| p.name == *name)
                    .ok_or(anyhow!("no preset named {} exists!", name))?;

                let mut config = config::get_config(&cli.config)?;

                config::print_userchrome(&preset, false);
                config.userchromes.push(preset);

                config::set_config(&cli.config, &config)?;
            } else {
                presets.into_iter().for_each(|p| {
                    config::print_userchrome(&p, true);
                })
            }

            Ok(())
        }

        Commands::Profile { path } => {
            if let Some(path) = path {
                let mut config = config::get_config(&cli.config)?;
                config.profile = path.to_owned();
                config::set_config(&cli.config, &config)?;
            } else {
                let config = config::get_config(&cli.config)?;
                println!(
                    "{}",
                    match !config.profile.is_empty() {
                        true => config.profile,
                        false => "not set".into(),
                    }
                );
            }

            Ok(())
        }

        Commands::Config { command } => match command {
            ConfigSubcommands::List { name } => {
                let config = config::get_config(&cli.config)?;
                let uc = config
                    .userchromes
                    .iter()
                    .find(|d| d.name.eq(name))
                    .ok_or(anyhow!("no userchrome with name {} exists", name))?;

                for c in &uc.configs {
                    println!("{}", config::format_userchrome_config(&c));
                }

                Ok(())
            }

            ConfigSubcommands::Set {
                name,
                key,
                value,
                raw,
            } => {
                let mut config = config::get_config(&cli.config)?;
                let chrome = config
                    .userchromes
                    .iter_mut()
                    .find(|d| d.name.eq(name))
                    .ok_or(anyhow!("no userchrome with name {} exists", name))?;

                let existing = chrome.configs.iter_mut().find(|c| c.key == *key);

                if let Some(existing) = existing {
                    existing.value = value.to_string();
                    existing.raw = *raw;
                } else {
                    chrome.configs.push(config::UserchromeConfig {
                        key: key.to_string(),
                        value: value.to_string(),
                        raw: *raw,
                    });
                }

                config::set_config(&cli.config, &config)?;

                Ok(())
            }

            ConfigSubcommands::Unset { name, key } => {
                let mut config = config::get_config(&cli.config)?;
                let chrome = config
                    .userchromes
                    .iter_mut()
                    .find(|d| d.name.eq(name))
                    .ok_or(anyhow!("no userchrome with name {} exists", name))?;

                let existing = chrome.configs.iter_mut().position(|c| c.key == *key);

                if let Some(existing) = existing {
                    chrome.configs.remove(existing);
                }

                config::set_config(&cli.config, &config)?;

                Ok(())
            }
        },

        Commands::Completions { shell } => {
            let cmd = &mut Cli::command();
            generate(*shell, cmd, cmd.get_name().to_string(), &mut io::stdout());

            Ok(())
        }
    }
}
