mod cli;
mod scanner;
mod models;

use clap::Parser;
use cli::Args;
use scanner::scan_active_connections;
use sysinfo::System;
use sysinfo::Pid;

fn main() {
    let args = Args::parse();
    let mut connections = scan_active_connections();
    
    let mut system = System::new();
    system.refresh_processes();

    for c in &mut connections {
        if let Some(proc) = system.process(Pid::from(c.pid as usize)) {
            c.process = proc.name().to_string();
        }
    }

    // 🔹 JSON MODE (machine-readable)
    if args.json {
        let json = serde_json::to_string_pretty(&connections)
            .expect("Failed to serialize connections");
        println!("{}", json);
        return;
    }

    // 🔹 DEFAULT MODE (human-readable)
    println!(
        "{:<6} {:<15} {:<22} {:<22} {}",
        "PID", "Process", "Local Address", "Remote Address", "Domain"
    );
    println!("{}", "-".repeat(80));

    for c in connections {
        println!(
            "{:<6} {:<15} {:<22} {:<22} {}",
            c.pid,
            c.process,
            c.local_addr,
            c.remote_addr,
            c.domain
        );
    }
}
