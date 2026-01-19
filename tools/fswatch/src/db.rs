use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::{Connection, Result};

pub fn db_path() -> PathBuf {
    let base = std::env::var("LOCALAPPDATA")
        .expect("LOCALAPPDATA not set");

    let mut path = PathBuf::from(base);
    path.push("Sentinel");
    path.push("fswatch");

    std::fs::create_dir_all(&path)
        .expect("Failed to create fswatch directory");

    path.push("fswatch.db");
    path
}

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

pub fn upsert_file(
    conn: &Connection,
    path: &str,
    size: u64,
    mtime: u64,
    hash: &str,
) -> Result<()> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    conn.execute(
        "
        INSERT INTO files (path, size, mtime, hash, last_seen)
        VALUES (?1, ?2, ?3, ?4, ?5)
        ON CONFLICT(path) DO UPDATE SET
            size = excluded.size,
            mtime = excluded.mtime,
            hash = excluded.hash,
            last_seen = excluded.last_seen;
        ",
        (path, size, mtime, hash, now),
    )?;

    Ok(())
}

pub fn get_file_meta(
    conn: &Connection,
    path: &str,
) -> Result<Option<(u64, u64)>> {
    let mut stmt = conn.prepare(
        "SELECT size, mtime FROM files WHERE path = ?1"
    )?;

    let mut rows = stmt.query([path])?;

    if let Some(row) = rows.next()? {
        let size: u64 = row.get(0)?;
        let mtime: u64 = row.get(1)?;
        Ok(Some((size, mtime)))
    } else {
        Ok(None)
    }
}

pub fn touch_file(conn: &Connection, path: &str) -> Result<()> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    conn.execute(
        "UPDATE files SET last_seen = ?1 WHERE path = ?2",
        (now, path),
    )?;

    Ok(())
}
