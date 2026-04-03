use std::path::Path;
use std::fs;

pub fn run() {
    let files = [
        "TANNER.png",
        "TANNER_ASCII.txt",
        "TANNER_ASCII_LARGE.txt",
    ];

    let mut found_paths = Vec::new();

    // Start from current directory
    if let Ok(current_dir) = std::env::current_dir() {
        search_directory(&current_dir, &files, &mut found_paths);
    }

    if found_paths.is_empty() {
        println!("No Tanner files found in the directory tree.");
    } else {
        println!("Found Tanner files:");
        for path in found_paths {
            println!("  {}", path.display());
        }
    }
}

fn search_directory(dir: &Path, target_files: &[&str], found: &mut Vec<std::path::PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                // Recurse into subdirectories
                search_directory(&path, target_files, found);
            } else if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                if target_files.contains(&file_name) {
                    found.push(path);
                }
            }
        }
    }
}