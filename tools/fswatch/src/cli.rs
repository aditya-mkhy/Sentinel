use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "fswatch")]
#[command(about = "File system integrity monitoring tool", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Scan filesystem and create/update baseline
    Scan {
        /// Path to scan
        path: PathBuf,

        /// Force re-hash all files
        #[arg(long)]
        force: bool,

        /// Output result as JSON
        #[arg(long)]
        json: bool,
    },

    /// Verify filesystem against stored baseline
    Verify {
        /// Path to verify
        path: PathBuf,

        /// Output result as JSON
        #[arg(long)]
        json: bool,
    },

    /// Show changes detected by watcher
    Show {
        /// Output result as JSON
        #[arg(long)]
        json: bool,
    },

    /// Show fswatch status
    Status,

    /// Initialize database and directories
    Init,
}
