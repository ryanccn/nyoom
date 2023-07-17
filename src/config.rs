use anyhow::Result;
use std::{fs, path};

use colored::*;
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
    pub clone_url: String,

    #[serde(default)]
    pub configs: Vec<UserchromeConfig>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    #[serde(default)]
    pub profile: String,

    #[serde(default)]
    pub userchromes: Vec<Userchrome>,
}

fn get_config_path() -> Result<path::PathBuf, Box<dyn std::error::Error>> {
    if let Some(config_dir) = dirs::config_dir() {
        Ok(config_dir.join("nyoom.toml"))
    } else {
        Err("unable to locate config dirs".into())
    }
}

pub fn get_config() -> Result<Config, Box<dyn std::error::Error>> {
    let path = get_config_path()?;
    let f = match path.exists() {
        true => fs::read_to_string(path)?,
        false => "".into(),
    };
    let config: Config = toml::from_str(&f)?;

    Ok(config)
}

pub fn set_config(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let serialized = toml::to_string_pretty(&config)?;
    fs::write(get_config_path()?, serialized)?;

    Ok(())
}

pub fn print_userchrome(userchrome: &Userchrome, short: bool) {
    println!(
        "{} {} {}",
        "·".cyan(),
        userchrome.name.cyan(),
        userchrome.clone_url.dimmed()
    );

    let slice_len = match short {
        true => userchrome.configs.len().min(3),
        false => userchrome.configs.len(),
    };

    for c in &userchrome.configs[..slice_len] {
        println!(
            "   {}: {}{}",
            c.key.magenta(),
            c.value,
            match c.raw {
                true => " (raw)".dimmed(),
                false => "".into(),
            }
        );
    }

    if short && userchrome.configs.len() > 3 {
        println!(
            "{}",
            format!("   and {} more", userchrome.configs.len() - 3).dimmed()
        );
    }
}
