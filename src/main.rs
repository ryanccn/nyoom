#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![deny(unsafe_code)]

mod cmd;
mod config;
mod presets;
mod switch;
mod utils;

use owo_colors::OwoColorize;

#[tokio::main]
async fn main() {
    if let Err(err) = cmd::main().await {
        println!("{}", "Encountered error:".red().bold());
        println!("{err}");
        std::process::exit(1);
    }
}
