use anyhow::{anyhow, Result};
use std::path;
use tokio::fs;

use owo_colors::OwoColorize;
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
    #[serde(default)]
    pub source: String,

    /// deprecated
    pub clone_url: Option<String>,

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

pub fn get_default_config_path() -> Result<path::PathBuf> {
    if let Some(config_dir) = dirs::config_dir() {
        Ok(config_dir.join("nyoom.toml"))
    } else {
        Err(anyhow!("unable to locate config dirs"))
    }
}

pub async fn get_config(path: &String) -> Result<Config> {
    let path_t = path::Path::new(path);

    let f = if path_t.exists() {
        fs::read_to_string(path).await?
    } else {
        String::new()
    };
    let mut config: Config = toml::from_str(&f)?;

    let mut migrated = false;

    for uc in &mut config.userchromes {
        if let Some(old_clone_url) = &uc.clone_url {
            uc.source = old_clone_url
                .replace("https://github.com/", "github:")
                .replace(".git", "");
            uc.clone_url = None;

            migrated = true;
        }
    }

    if migrated {
        set_config(path, &config).await?;
    }

    Ok(config)
}

pub async fn set_config(path: &String, config: &Config) -> Result<()> {
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
