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
        clone_url: String,
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
        #[arg(short, long, value_hint = ValueHint::DirPath)]
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
        shell: String,
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
            let config = config::get_config()?;
            for u in config.userchromes {
                config::print_userchrome(&u, false);
            }

            Ok(())
        }

        Commands::Add { name, clone_url } => {
            let mut config = config::get_config()?;

            let new_userchrome = config::Userchrome {
                name: name.to_string(),
                clone_url: clone_url.to_string(),
                configs: vec![],
            };
            config::print_userchrome(&new_userchrome, false);
            config.userchromes.push(new_userchrome);

            config::set_config(config)?;

            Ok(())
        }

        Commands::Switch { name } => {
            let config = config::get_config()?;
            match config.userchromes.iter().find(|c| c.name.eq(name)) {
                Some(u) => {
                    if config.profile == "" {
                        panic!("no profile configured")
                    }

                    switch::switch(u, config.profile)?;
                }
                None => {
                    panic!("no userchrome with name {} found!", name)
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

                let mut config = config::get_config()?;

                config.userchromes.push(preset);

                config::set_config(config)?;
            } else {
                presets.into_iter().for_each(|p| {
                    config::print_userchrome(&p, true);
                })
            }

            Ok(())
        }

        Commands::Profile { path } => {
            if let Some(path) = path {
                let mut config = config::get_config()?;
                config.profile = path.to_owned();
                config::set_config(config)?;
            } else {
                let config = config::get_config()?;
                println!(
                    "{}",
                    match config.profile != "" {
                        true => config.profile,
                        false => "not set".into(),
                    }
                );
            }

            Ok(())
        }

        Commands::Config { command } => match command {
            ConfigSubcommands::List { name } => {
                let config = config::get_config()?;
                let uc = config
                    .userchromes
                    .iter()
                    .find(|d| d.name.eq(name))
                    .ok_or(anyhow!("no userchrome with name {} exists", name))?;

                for c in &uc.configs {
                    println!("{} = {} (raw: {})", c.key, c.value, c.raw);
                }

                Ok(())
            }

            ConfigSubcommands::Set {
                name,
                key,
                value,
                raw,
            } => {
                let mut config = config::get_config()?;
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

                config::set_config(config)?;

                Ok(())
            }

            ConfigSubcommands::Unset { name, key } => {
                let mut config = config::get_config()?;
                let chrome = config
                    .userchromes
                    .iter_mut()
                    .find(|d| d.name.eq(name))
                    .ok_or(anyhow!("no userchrome with name {} exists", name))?;

                let existing = chrome.configs.iter_mut().position(|c| c.key == *key);

                if let Some(existing) = existing {
                    chrome.configs.remove(existing);
                }

                config::set_config(config)?;

                Ok(())
            }
        },

        Commands::Completions { shell } => {
            let generator = match shell.as_str() {
                "bash" => Ok(Shell::Bash),
                "zsh" => Ok(Shell::Zsh),
                "elvish" => Ok(Shell::Elvish),
                "fish" => Ok(Shell::Fish),
                "pwsh" => Ok(Shell::PowerShell),
                "powershell" => Ok(Shell::PowerShell),
                &_ => Err(anyhow!("{} is not a valid shell", shell)),
            }?;

            let cmd = &mut Cli::command();

            generate(
                generator,
                cmd,
                cmd.get_name().to_string(),
                &mut io::stdout(),
            );

            Ok(())
        }
    }
}
