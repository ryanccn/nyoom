// SPDX-FileCopyrightText: 2024 Ryan Cao <hello@ryanccn.dev>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use etcetera::AppStrategy as _;
use eyre::{eyre, Result};
use std::path::{Path, PathBuf};
use tokio::fs;

use owo_colors::OwoColorize as _;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct UserchromeConfig {
    pub key: String,
    pub value: String,
    pub raw: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Userchrome {
    pub name: String,
    pub source: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub clone_url: Option<String>,

    #[serde(default)]
    pub configs: Vec<UserchromeConfig>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Config {
    pub profile: Option<PathBuf>,

    #[serde(default)]
    pub userchromes: Vec<Userchrome>,
}

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

pub async fn get_config(path: &Path) -> Result<Config> {
    match fs::read_to_string(path).await {
        Ok(s) => toml::from_str(&s).map_err(Into::into),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Config::default()),
        Err(e) => Err(e.into()),
    }
}

pub async fn set_config(path: &Path, config: &Config) -> Result<()> {
    fs::create_dir_all(
        path.parent()
            .ok_or_else(|| eyre!("Could not obtain parent directory of config"))?,
    )
    .await?;

    let serialized = toml::to_string_pretty(&config)?;
    fs::write(path, serialized).await?;

    Ok(())
}

pub fn format_userchrome_config(c: &UserchromeConfig) -> String {
    format!(
        "{}: {}{}",
        c.key.magenta(),
        c.value,
        if c.raw {
            " (raw)".dimmed().to_string()
        } else {
            String::new()
        }
    )
}

pub fn print_userchrome(userchrome: &Userchrome, short: bool) {
    println!(
        "{} {} {}",
        "·".cyan(),
        userchrome.name.cyan(),
        userchrome.source.dimmed()
    );

    let slice_len = if short {
        userchrome.configs.len().min(3)
    } else {
        userchrome.configs.len()
    };

    for c in &userchrome.configs[..slice_len] {
        println!("    {}", format_userchrome_config(c));
    }

    if short && userchrome.configs.len() > 3 {
        println!(
            "{}",
            format!("    and {} more", userchrome.configs.len() - 3).dimmed()
        );
    }
}
