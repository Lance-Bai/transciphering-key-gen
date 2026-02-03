use std::env;
use std::fs;
use std::path::Path;

use submission::help_fun::get_size_string;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <size>", args[0]);
        std::process::exit(1); 
    }
    let size = args[1].clone();
    let io_dir = "io/".to_owned() + get_size_string(size.parse::<usize>()?);
    
    let source_dir = format!("{}/ciphertext_aes_download", io_dir);
    let target_dir = format!("{}/ciphertexts_download", io_dir);

    // Create target directory if it doesn't exist
    fs::create_dir_all(&target_dir)?;

    // Copy all files from source to target
    if Path::new(&source_dir).exists() {
        for entry in fs::read_dir(&source_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                let file_name = path.file_name().unwrap();
                let target_path = format!("{}/{}", target_dir, file_name.to_string_lossy());
                fs::copy(&path, &target_path)?;
            }
        }
    }
    Ok(())
}