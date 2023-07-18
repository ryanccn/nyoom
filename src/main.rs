mod cmd;
mod config;
mod presets;
mod switch;
mod utils;

use colored::*;

fn main() {
    if let Err(err) = cmd::main() {
        println!("{}", "Encountered error:".red().bold());
        println!("{}", err);
        std::process::exit(1);
    }
}
