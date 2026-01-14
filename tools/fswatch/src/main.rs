mod cli;
mod scanner;
mod hasher;
mod db;

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
            match db::init_db() {
                Ok(_) => println!("Database initialized"),
                Err(e) => eprintln!("Failed to initialize database: {}", e),
            }
        }
    }
}