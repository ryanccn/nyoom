use rust_embed::RustEmbed;

use crate::config;

#[derive(RustEmbed)]
#[folder = "presets/"]
struct Presets;

pub fn get_presets() -> Vec<config::Userchrome> {
    Presets::iter()
        .filter(|f| f.ends_with(".toml"))
        .map(|f| Presets::get(&f).unwrap().data)
        .map(|f| String::from_utf8(f.to_owned().into()).unwrap())
        .map(|f| toml::from_str(&f).unwrap())
        .collect()
}
