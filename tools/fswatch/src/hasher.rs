use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

const HASH_CHUNK_SIZE: usize = 10 * 1024 * 1024; // 3 MB

/// Compute SHA-256 hash of a file (streaming, memory-safe)
pub fn hash_file(path: &Path) -> Result<String, std::io::Error> {
    let file = File::open(path)?;
    let mut reader = BufReader::with_capacity(HASH_CHUNK_SIZE, file);

    let mut hasher = Sha256::new();
    let mut buffer = vec![0u8; HASH_CHUNK_SIZE];


    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let hash = hasher.finalize();
    Ok(format!("{:x}", hash))
}