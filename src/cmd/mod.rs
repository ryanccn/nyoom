use clap::{CommandFactory, Parser, Subcommand, ValueHint};
use clap_complete::{generate, Shell};

use std::io;

use crate::{config, switch};

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

pub fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::List {} => {
            let config = config::get_config();
            for u in config.userchromes {
                println!("{} -> {}", u.name, u.clone_url);
            }
        }

        Commands::Add { name, clone_url } => {
            let mut config = config::get_config();

            config.userchromes.push(config::Userchrome {
                name: name.to_string(),
                clone_url: clone_url.to_string(),
                configs: vec![],
            });

            config::set_config(config);
        }

        Commands::Switch { name } => {
            let config = config::get_config();
            match config.userchromes.iter().find(|c| c.name.eq(name)) {
                Some(u) => {
                    switch::switch(u, config.profile);
                }
                None => {
                    panic!("no userchrome with name {} found!", name)
                }
            };
        }

        Commands::Profile { path } => {
            if let Some(path) = path {
                let mut config = config::get_config();
                config.profile = path.to_owned();
                config::set_config(config);
            } else {
                let config = config::get_config();
                println!("{}", config.profile);
            }
        }

        Commands::Config { command } => match command {
            ConfigSubcommands::List { name } => {
                let config = config::get_config();
                let uc = config
                    .userchromes
                    .iter()
                    .find(|d| d.name.eq(name))
                    .expect(&format!("no userchrome with name {} exists", name));

                for c in &uc.configs {
                    println!("{} = {} (raw: {})", c.key, c.value, c.raw);
                }
            }

            ConfigSubcommands::Set {
                name,
                key,
                value,
                raw,
            } => {
                let mut config = config::get_config();
                let chrome = config
                    .userchromes
                    .iter_mut()
                    .find(|d| d.name.eq(name))
                    .expect(&format!("no userchrome with name {} exists", name));

                let existing = chrome.configs.iter_mut().find(|c| c.key == key.to_string());

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

                config::set_config(config);
            }

            ConfigSubcommands::Unset { name, key } => {
                let mut config = config::get_config();
                let chrome = config
                    .userchromes
                    .iter_mut()
                    .find(|d| d.name.eq(name))
                    .expect(&format!("no userchrome with name {} exists", name));

                let existing = chrome
                    .configs
                    .iter_mut()
                    .position(|c| c.key == key.to_string());

                if let Some(existing) = existing {
                    chrome.configs.remove(existing);
                }

                config::set_config(config);
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
                &_ => Err(format!("{} is not a valid shell", shell)),
            }
            .unwrap();

            let cmd = &mut Cli::command();

            generate(
                generator,
                cmd,
                cmd.get_name().to_string(),
                &mut io::stdout(),
            );
        }
    }
}
