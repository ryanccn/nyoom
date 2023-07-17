use std::{fs, path};

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
