use clap::{Parser, Subcommand};

use crate::{config, switch};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// list userchromes
    List {},

    /// add a new userchrome
    Add { name: String, clone_url: String },

    /// switch to a userchrome
    Switch { name: String },

    /// configure Fierfox profile or get current directory
    Profile { path: Option<String> },
}

pub fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::List {} => {
            let config = config::get_config();
            for u in config.userchromes {
                println!("{} -> {}", u.name, u.clone_url);
            }
        }

        Commands::Add { name, clone_url } => {
            let mut config = config::get_config();

            config.userchromes.push(config::Userchrome {
                name: name.to_string(),
                clone_url: clone_url.to_string(),
                configs: vec![],
            });

            config::set_config(config);
        }

        Commands::Switch { name } => {
            let config = config::get_config();
            match config.userchromes.iter().find(|c| c.name.eq(name)) {
                Some(u) => {
                    switch::switch(u, config.profile);
                }
                None => {
                    panic!("no userchrome with name {} found!", name)
                }
            };
        }

        Commands::Profile { path } => {
            if let Some(path) = path {
                let mut config = config::get_config();
                config.profile = path.to_owned();
                config::set_config(config);
            } else {
                let config = config::get_config();
                println!("{}", config.profile);
            }
        }
    }
}
