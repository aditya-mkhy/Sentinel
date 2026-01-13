mod cli;
mod scanner;
mod hasher;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { path, .. } => {
            println!("Starting full scan: {:?}", path);
            scanner::walk_and_scan(&path);
        }

        Commands::Verify { .. } => {
            println!("Verify not implemented yet");
        }

        Commands::Show { .. } => {
            println!("Show not implemented yet");
        }

        Commands::Status => {
            println!("Status not implemented yet");
        }

        Commands::Init => {
            println!("Init not implemented yet");
        }
    }
}