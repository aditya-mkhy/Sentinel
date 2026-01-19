use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::Connection;

use crate::db;
use crate::hasher;

/// Walk filesystem, extract metadata, hash files, and insert into DB
pub fn walk_and_scan(path: &Path, conn: &Connection) {
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
            walk_and_scan(&entry_path, conn);
        } else if entry_path.is_file() {
            if let Err(e) = scan_file(&entry_path, conn) {
                eprintln!("Failed to process {:?}: {}", entry_path, e);
            }
        }
    }
}

fn scan_file(path: &Path, conn: &Connection) -> rusqlite::Result<()> {
    let metadata = match fs::metadata(path) {
        Ok(m) => m,
        Err(_) => return Ok(()), // silently skip unreadable files
    };

    let size = metadata.len();

    let modified = metadata
        .modified()
        .map(systemtime_to_unix)
        .unwrap_or(0);

    let path_str = path.to_string_lossy();

    if let Ok(Some((old_size, old_mtime))) =
        db::get_file_meta(conn, &path_str)
    {
        if old_size == size && old_mtime == modified {
            // File unchanged → skip hashing
            db::touch_file(conn, &path_str)?;
            return Ok(());
        }
    }

    // File is new or changed → hash
    let hash = hasher::hash_file(path)?;

    db::upsert_file(
        conn,
        &path_str,
        size,
        modified,
        &hash,
    )?;

    println!(
        "{} | size={} | mtime={} | hash={}",
        path.display(),
        size,
        modified,
        hash
    );

    Ok(())
}

fn systemtime_to_unix(time: SystemTime) -> u64 {
    time.duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}