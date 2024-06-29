use crate::config::get_default_config_path;
use clap::{Parser, Subcommand, ValueHint};

use async_trait::async_trait;
use color_eyre::eyre::Result;
use enum_dispatch::enum_dispatch;

mod add;
mod completions;
mod config;
mod list;
mod preset;
mod profile;
mod remove;
mod switch;

#[derive(Parser)]
#[command(author, version, about = "\x1B[36;1mnyoom · Firefox userchrome manager\x1B[0m", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Config file to use
    #[arg(short, long, default_value_t = get_default_config_path().unwrap().into_os_string().into_string().unwrap(), value_hint = ValueHint::FilePath)]
    config: String,
}

#[async_trait]
#[enum_dispatch]
pub trait Command {
    async fn action(&self, global_options: &Cli) -> Result<()>;
}

#[derive(Subcommand)]
#[enum_dispatch(Command)]
pub enum Commands {
    /// List userchromes
    List(list::ListCommand),
    /// Add a new userchrome
    Add(add::AddCommand),
    /// Remove a userchrome
    Remove(remove::RemoveCommand),
    /// Switch to a userchrome
    Switch(switch::SwitchCommand),
    /// Import a preset as a userchrome or list presets
    Preset(preset::PresetCommand),
    /// Configure Firefox profile or get current directory
    Profile(profile::ProfileCommand),
    /// Manage userchrome-linked configs
    Config(config::ConfigCommand),
    /// Generate completions
    Completions(completions::CompletionCommand),
}
