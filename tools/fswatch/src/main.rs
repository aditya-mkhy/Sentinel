mod cli;
mod scanner;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { path, force, json } => {
            println!("scan -> path: {:?}, force: {}, json: {}", path, force, json);
            println!("Scanning path: {:?}", path);
            scanner::walk_and_print(&path);
        }
        Commands::Verify { path, json } => {
            println!("verify -> path: {:?}, json: {}", path, json);
        }
        Commands::Show { json } => {
            println!("show -> json: {}", json);
        }
        Commands::Status => {
            println!("status");
        }
        Commands::Init => {
            println!("init");
        }
    }
}
