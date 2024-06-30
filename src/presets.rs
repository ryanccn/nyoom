use color_eyre::eyre::{eyre, Result};
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
                    .ok_or_else(|| {
                        eyre!(
                            "preset {} not found despite it being returned from iterator",
                            f
                        )
                    })?
                    .data
                    .into_owned(),
            )?)?)
        })
        .collect()
}
