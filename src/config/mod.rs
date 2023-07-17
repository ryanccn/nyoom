use std::fs;

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
    pub profile: String,
    pub userchromes: Vec<Userchrome>,
}

fn get_config_path() -> String {
    String::from(
        dirs::config_dir()
            .unwrap()
            .join("nyoom.toml")
            .to_str()
            .unwrap(),
    )
}

pub fn get_config() -> Config {
    let f = fs::read_to_string(get_config_path()).unwrap();
    let config: Config = toml::from_str(&f).unwrap();

    config
}

pub fn set_config(config: Config) {
    let serialized = toml::to_string_pretty(&config).unwrap();
    fs::write(get_config_path(), serialized).unwrap();
}
