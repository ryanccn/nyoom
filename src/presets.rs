use anyhow::{anyhow, Result};
use rust_embed::RustEmbed;

use crate::config::Userchrome;

#[derive(RustEmbed)]
#[folder = "presets/"]
struct Presets;

pub fn get_presets() -> Result<Vec<Userchrome>> {
    Presets::iter()
        .filter(|f| f.ends_with(".toml"))
        .map(|f| -> Result<Userchrome> {
            Ok(toml::from_str(&String::from_utf8(
                Presets::get(&f)
                    .ok_or(anyhow!("this isn't supposed to happen"))?
                    .data
                    .into_owned(),
            )?)?)
        })
        .collect()
}
