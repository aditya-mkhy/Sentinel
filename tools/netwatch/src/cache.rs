use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use dirs::data_local_dir;

const CACHE_FILE: &str = "dns_cache.json";

fn cache_path() -> Option<PathBuf> {
    let mut path = data_local_dir()?;
    path.push("Sentinel");
    path.push("netwatch");
    fs::create_dir_all(&path).ok()?;
    path.push(CACHE_FILE);
    Some(path)
}

pub fn load_dns_cache() -> HashMap<String, String> {
    let path = match cache_path() {
        Some(p) => p,
        None => return HashMap::new(),
    };

    let data = match fs::read_to_string(path) {
        Ok(d) => d,
        Err(_) => return HashMap::new(),
    };

    serde_json::from_str(&data).unwrap_or_default()
}

pub fn save_dns_cache(cache: &HashMap<String, String>) {
    let path = match cache_path() {
        Some(p) => p,
        None => return,
    };

    if let Ok(json) = serde_json::to_string_pretty(cache) {
        let _ = fs::write(path, json);
    }
}