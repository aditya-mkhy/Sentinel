use std::fs;
use std::path::Path;

/// Walk a directory recursively and print all files
pub fn walk_and_print(path: &Path) {
    // Try to read directory
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

        // If directory → recurse
        if entry_path.is_dir() {
            walk_and_print(&entry_path);
        }
        // If file → print
        else if entry_path.is_file() {
            println!("{}", entry_path.display());
        }
    }
}
