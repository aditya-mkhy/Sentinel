use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

/// Walk a directory recursively and print file metadata
pub fn walk_and_print(path: &Path) {
    let entries = match fs::read_dir(path) {
        Ok(e) => e,
        Err(err) => {
            eprintln!("Cannot read directory {:?}: {}", path, err);
            return;
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(err) => {
                eprintln!("Failed to read entry: {}", err);
                continue;
            }
        };

        let entry_path = entry.path();

        if entry_path.is_dir() {
            walk_and_print(&entry_path);
        } else if entry_path.is_file() {
            print_file_metadata(&entry_path);
        }
    }
}

fn print_file_metadata(path: &Path) {
    let metadata = match fs::metadata(path) {
        Ok(m) => m,
        Err(err) => {
            eprintln!("Cannot read metadata {:?}: {}", path, err);
            return;
        }
    };

    let size = metadata.len();

    let modified = match metadata.modified() {
        Ok(time) => systemtime_to_unix(time),
        Err(_) => 0,
    };

    println!(
        "{} | size={} bytes | mtime={}",
        path.display(),
        size,
        modified
    );
}

fn systemtime_to_unix(time: SystemTime) -> u64 {
    time.duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}
