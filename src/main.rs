mod cli;
mod assets;
mod signature;
mod commands;

use clap::Parser;
use cli::{Cli, Commands};



fn main() {
    let cli: Cli = Cli::parse();

    match cli.command {
        Commands::Init { picture, normal, enlarged, force } => {
            commands::init::run(picture, normal, enlarged, force);
        }
        Commands::Copy { small, large } => {
            commands::copy::run(small, large);
        }
        Commands::Verify => {
            commands::verify::run();
        }
        Commands::Remove => {
            println!("Remove logic coming soon.");
        }
    }
}
