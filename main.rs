use std::collections::HashMap;
use std::fs::File;
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

fn find_duplicate_files(directory_path: &str) -> io::Result<Vec<String>> {
    let mut file_hashes: HashMap<u64, Vec<String>> = HashMap::new();
    let mut duplicate_files: Vec<String> = Vec::new();

    for entry in WalkDir::new(directory_path).into_iter().filter_map(Result::ok).filter(|entry| entry.path().is_file()) {
        let file_path = entry.path().to_str().unwrap();

        match calculate_file_hash(file_path) {
            Ok(file_hash) => {
                if let Some(paths) = file_hashes.get_mut(&file_hash) {
                    paths.push(file_path.to_string());
                    duplicate_files.push(file_path.to_string());
                } else {
                    file_hashes.insert(file_hash, vec![file_path.to_string()]);
                }
            },
            Err(error) => eprintln!("Error hashing file {}: {}", file_path, error),
        }
    }

    // Print all files with the same hash
    for (hash, paths) in file_hashes.iter() {
        if paths.len() > 1 {
            println!("Files with hash {}: {:?}", hash, paths);
        }
    }

    Ok(duplicate_files)
}

fn main() {
    let command_line_args: Vec<String> = std::env::args().collect();
    if command_line_args.len() != 2 {
        eprintln!("Usage: {} <path_to_directory>", command_line_args[0]);
        std::process::exit(1);
    }

    let directory_path = &command_line_args[1];

    match find_duplicate_files(directory_path) {
        Ok(duplicate_files) => {
            if duplicate_files.is_empty() {
                println!("No duplicate files found.");
            } else {
                println!("Duplicate files found:");
                for duplicate_file in duplicate_files {
                    println!("{}", duplicate_file);
                }
            }
        },
        Err(error) => eprintln!("Error: {}", error),
    }
}

// use std::fs::{self, File};
// use std::io::Write;
// use std::path::Path;
// use rand::Rng;
// use rand::distributions::Alphanumeric;

// fn create_random_text_file(directory: &Path, file_number: usize) -> std::io::Result<()> {
//     let file_name = format!("file_{}.txt", file_number);
//     let file_path = directory.join(file_name);
//     let mut file = File::create(file_path)?;

//     let random_content: String = rand::thread_rng()
//         .sample_iter(&Alphanumeric)
//         .take(100)  // Change this value to increase or decrease the length of the content
//         .map(char::from)
//         .collect();

//     file.write_all(random_content.as_bytes())?;
//     Ok(())
// }

// fn main() -> std::io::Result<()> {
//     let directory = Path::new("/projects/web-nodejs-sample/duplicate_file_analyzer/public/folder3/folder3-2");

//     // Create the directory if it doesn't exist
//     if !directory.exists() {
//         fs::create_dir_all(directory)?;
//     }

//     // Number of text files to create
//     let num_files = 10;

//     for i in 0..num_files {
//         create_random_text_file(&directory, i)?;
//     }

//     println!("Created {} text files in {:?}", num_files, directory);
//     Ok(())
// }
