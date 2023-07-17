mod cmd;
mod config;
mod presets;
mod switch;

use colored::*;

fn main() {
    if let Err(err) = cmd::main() {
        println!("{}", "Encountered error:".red().bold());
        println!("{}", err);
    }
}
