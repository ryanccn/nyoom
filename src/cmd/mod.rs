use clap::{Parser, Subcommand};

use crate::{config, util};

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
}

pub fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::List {} => {
            let config = crate::config::get_config();
            for u in config.userchromes {
                println!("{} -> {}", u.name, u.clone_url);
            }
        }

        Commands::Add { name, clone_url } => {
            let mut config = crate::config::get_config();

            config.userchromes.push(config::Userchrome {
                name: name.to_string(),
                clone_url: clone_url.to_string(),
                configs: vec![],
            });

            config::set_config(config);
        }

        Commands::Switch { name } => {
            let config = crate::config::get_config();
            match config.userchromes.iter().find(|c| c.name.eq(name)) {
                Some(u) => {
                    util::switch::switch(u, config.profile);
                }
                None => {
                    panic!("No userchrome with name {} found!", name)
                }
            };
        }
    }
}
