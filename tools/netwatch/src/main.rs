mod cli;
mod scanner;
mod models;
mod resolver;
mod cache;

use clap::Parser;
use cli::Args;
use scanner::scan_active_connections;
use sysinfo::System;
use sysinfo::Pid;
use std::collections::HashMap;

fn main() {
    let args = Args::parse();
    let mut connections = scan_active_connections();

    let mut system = System::new();
    system.refresh_processes();

    let mut dns_cache: HashMap<String, String> = cache::load_dns_cache();

    for c in &mut connections {
        // process name
        if let Some(proc) = system.process(Pid::from(c.pid as usize)) {
            c.process = proc.name().to_string();
        }

        // Reverse DNS with cache
        let remote_ip = c
            .remote_addr
            .split(':')
            .next()
            .unwrap_or("")
            .to_string();

        if let Some(cached) = dns_cache.get(&remote_ip) {
            c.domain = cached.clone();
        } else {
            let domain = resolver::reverse_dns(&remote_ip)
                .unwrap_or_else(|| "-".to_string());

            dns_cache.insert(remote_ip.clone(), domain.clone());
            c.domain = domain;
        }

    }

    cache::save_dns_cache(&dns_cache); // save dns_cache

    // JSON MODE (machine-readable)
    if args.json {
        let json = serde_json::to_string_pretty(&connections)
            .expect("Failed to serialize connections");
        println!("{}", json);
        return;
    }

    // DEFAULT MODE (human-readable)
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
