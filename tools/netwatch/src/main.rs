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

fn main() {
    let args = Args::parse();

    let resolve = args.resolve || args.resolve_refresh;
    let refresh = args.resolve_refresh;

    let mut connections = scan_active_connections();

    let mut system = System::new();
    system.refresh_processes();

    // Load cache ONLY when needed
    let mut dns_cache = if resolve && !refresh {
        cache::load_dns_cache()
    } else {
        std::collections::HashMap::new()
    };


    for c in &mut connections {
        // process name
        if let Some(proc) = system.process(Pid::from(c.pid as usize)) {
            c.process = proc.name().to_string();
        }

        if !resolve {
            continue;
        }

        // Reverse DNS with cache
        let remote_ip = c
            .remote_addr
            .split(':')
            .next()
            .unwrap_or("")
            .to_string();

        if !refresh {
            if let Some(cached) = dns_cache.get(&remote_ip) {
                c.domain = cached.clone();
                continue;
            }
        }

        let domain = resolver::reverse_dns(&remote_ip)
            .unwrap_or_else(|| "-".to_string());

        dns_cache.insert(remote_ip, domain.clone());
        c.domain = domain;

    }

    if resolve {
        cache::save_dns_cache(&dns_cache); // save dns_cache
    }


    // JSON MODE
    if args.json {
        let json = serde_json::to_string_pretty(&connections)
            .expect("Failed to serialize connections");
        println!("{}", json);
        return;
    }

    // DEFAULT MODE
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
