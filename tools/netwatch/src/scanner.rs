// src/scanner.rs

use crate::models::Connection;

pub fn scan_active_connections() -> Vec<Connection> {
    vec![
        Connection {
            pid: 6460,
            process: "chrome.exe".to_string(),
            local_addr: "10.0.0.3:52765".to_string(),
            remote_addr: "140.82.113.25:443".to_string(),
            domain: "-".to_string(),
        },
    ]
}
