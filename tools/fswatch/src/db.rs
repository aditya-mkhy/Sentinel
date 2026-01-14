use std::path::PathBuf;
use rusqlite::{Connection, Result};

pub fn init_db() -> Result<()> {
    let path = db_path();
    let conn = Connection::open(path)?;

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS files (
            path TEXT PRIMARY KEY,
            size INTEGER NOT NULL,
            mtime INTEGER NOT NULL,
            hash TEXT NOT NULL,
            last_seen INTEGER NOT NULL
        );
        ",
        [],
    )?;

    Ok(())
}


fn db_path() -> PathBuf {
    let base = std::env::var("LOCALAPPDATA")
        .expect("LOCALAPPDATA not set");

    let mut path = PathBuf::from(base);
    path.push("Sentinel");
    path.push("fswatch");

    std::fs::create_dir_all(&path).expect("Failed to create fswatch directory");

    path.push("fswatch.db");
    path
}