use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Read};
use walkdir::WalkDir;
use twox_hash::XxHash64;
use std::hash::Hasher;

fn calculate_file_hash(file_path: &str) -> io::Result<u64> {
    let mut file = File::open(file_path)?;
    let mut hasher = XxHash64::default();
    let mut read_buffer = [0; 8192]; // Increase buffer size for better performance
    loop {
        let bytes_read = file.read(&mut read_buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.write(&read_buffer[..bytes_read]);
    }
    Ok(hasher.finish())
}

fn find_duplicate_files(directory_path: &str) -> io::Result<HashMap<u64, Vec<String>>> {
    let mut file_hashes: HashMap<u64, Vec<String>> = HashMap::new();

    for entry in WalkDir::new(directory_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_file()) 
    {
        let file_path = entry.path().to_str().unwrap();

        match calculate_file_hash(file_path) {
            Ok(file_hash) => {
                file_hashes
                    .entry(file_hash)
                    .or_insert_with(Vec::new)
                    .push(file_path.to_string());
            },
            Err(error) => eprintln!("Error hashing file {}: {}", file_path, error),
        }
    }

    Ok(file_hashes)
}

fn delete_duplicate_files(file_hashes: &HashMap<u64, Vec<String>>) {
    for (hash, paths) in file_hashes {
        if paths.len() > 1 {
            println!("\nFiles with same hash {}:\n{}", hash, paths.join("\n"));
            for path in &paths[1..] {
                if let Err(error) = fs::remove_file(path) {
                    eprintln!("Error deleting file {}: {}", path, error);
                } else {
                    println!("Deleted duplicate file: {}", path);
                }
            }
        }
    }
}

fn main() {
    let command_line_args: Vec<String> = std::env::args().collect();
    if command_line_args.len() != 2 {
        eprintln!("Usage: {} <path_to_directory>", command_line_args[0]);
        std::process::exit(1);
    }

    let directory_path = &command_line_args[1];

    match find_duplicate_files(directory_path) {
        Ok(file_hashes) => {
            let mut duplicates_found = false;
            for (hash, paths) in &file_hashes {
                if paths.len() > 1 {
                    duplicates_found = true;
                    println!("\nFiles with same hash {}:\n{}", hash, paths.join("\n"));
                }
            }
            if !duplicates_found {
                println!("No duplicate files found.");
            } else {
                println!("Would you like to delete the duplicate files? (yes/no): ");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                if input.trim().eq_ignore_ascii_case("yes") {
                    delete_duplicate_files(&file_hashes);
                } else {
                    println!("Duplicate files not deleted.");
                }
            }
        },
        Err(error) => eprintln!("Error: {}", error),
    }
}
