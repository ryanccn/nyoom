use color_eyre::eyre::{eyre, Result};
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

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    #[serde(default)]
    pub profile: String,

    #[serde(default)]
    pub userchromes: Vec<Userchrome>,
}

pub fn get_old_config_path() -> Result<PathBuf> {
    dirs::config_dir()
        .ok_or_else(|| eyre!("Unable to locate config dirs"))
        .map(|dir| dir.join("nyoom.toml"))
}

pub fn get_default_config_path() -> Result<PathBuf> {
    dirs::config_dir()
        .ok_or_else(|| eyre!("Unable to locate config dirs"))
        .map(|dir| dir.join("nyoom").join("nyoom.toml"))
}

pub async fn migrate_config() -> Result<()> {
    let old = get_old_config_path()?;
    let new = get_default_config_path()?;

    if old.exists() && !new.exists() {
        fs::create_dir_all(
            new.parent()
                .ok_or_else(|| eyre!("Could not obtain parent directory of config"))?,
        )
        .await?;
        fs::copy(old, new).await?;
    }

    Ok(())
}

pub async fn get_config(path: &Path) -> Result<Config> {
    let f = if Path::new(path).exists() {
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
