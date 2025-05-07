// SPDX-FileCopyrightText: 2024 Ryan Cao <hello@ryanccn.dev>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use eyre::{Result, eyre};
use std::{
    fmt,
    path::{Path, PathBuf},
};
use tokio::fs;

use anstream::println;
use etcetera::AppStrategy as _;
use owo_colors::OwoColorize as _;
use serde::{Deserialize, Serialize};

fn strategy() -> Result<impl etcetera::AppStrategy> {
    etcetera::choose_app_strategy(etcetera::AppStrategyArgs {
        top_level_domain: "dev.ryanccn".to_owned(),
        author: "Ryan Cao".to_owned(),
        app_name: "nyoom".to_owned(),
    })
    .map_err(|e| e.into())
}

pub fn get_default_config_path() -> Result<PathBuf> {
    Ok(strategy()?.config_dir().join("nyoom.toml"))
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UserchromeConfig {
    pub key: String,
    pub value: String,
    pub raw: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Userchrome {
    pub name: String,
    pub source: String,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub configs: Vec<UserchromeConfig>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Config {
    pub profile: Option<PathBuf>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub userchromes: Vec<Userchrome>,
}

impl Config {
    pub async fn read(path: &Path) -> Result<Self> {
        match fs::read_to_string(path).await {
            Ok(s) => toml::from_str(&s).map_err(|e| e.into()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Config::default()),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn write(&self, path: &Path) -> Result<()> {
        fs::create_dir_all(
            path.parent()
                .ok_or_else(|| eyre!("could not obtain parent directory of config"))?,
        )
        .await?;

        let serialized = toml::to_string_pretty(&self)?;
        fs::write(path, serialized).await?;

        Ok(())
    }
}

impl fmt::Display for UserchromeConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}{}",
            self.key.magenta(),
            self.value,
            if self.raw {
                " (raw)".dimmed().to_string()
            } else {
                String::new()
            }
        )
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum PrintContext {
    #[default]
    Normal,
    Modified,
    Added,
    Removed,
}

impl Userchrome {
    pub fn print(&self, short: bool, context: PrintContext) {
        match context {
            PrintContext::Normal => {
                println!(
                    "{} {} {}",
                    "·".cyan(),
                    self.name.cyan(),
                    self.source.dimmed()
                );
            }
            PrintContext::Modified => {
                println!(
                    "{} {} {}",
                    "*".blue(),
                    self.name.blue(),
                    self.source.dimmed()
                );
            }
            PrintContext::Added => {
                println!(
                    "{} {} {}",
                    "+".green(),
                    self.name.green(),
                    self.source.dimmed()
                );
            }
            PrintContext::Removed => {
                println!("{} {} {}", "-".red(), self.name.red(), self.source.dimmed());
            }
        }

        for c in if short {
            &self.configs[..self.configs.len().min(3)]
        } else {
            &self.configs
        } {
            println!("    {}", c.to_string());
        }

        if short && self.configs.len() > 3 {
            println!(
                "{}",
                format!("    and {} more", self.configs.len() - 3).dimmed()
            );
        }
    }
}
