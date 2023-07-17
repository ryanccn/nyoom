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

fn get_config_path() -> path::PathBuf {
    dirs::config_dir().unwrap().join("nyoom.toml")
}

pub fn get_config() -> Config {
    let path = get_config_path();
    let f = match path.exists() {
        true => fs::read_to_string(path).unwrap(),
        false => "".into(),
    };
    let config: Config = toml::from_str(&f).unwrap();

    config
}

pub fn set_config(config: Config) {
    let serialized = toml::to_string_pretty(&config).unwrap();
    fs::write(get_config_path(), serialized).unwrap();
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
