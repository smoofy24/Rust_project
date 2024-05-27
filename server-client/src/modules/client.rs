use log::{error, info, warn};
use std::io::{self, Write};
use std::fs;
use std::path::Path;

pub fn is_valid_file(path: &str) -> bool {
    let path = Path::new(path);

    // Check if the path exists and is a file
    if !path.exists() || !path.is_file() {
        return false;
    }

    // Check if the file is readable
    match fs::File::open(path) {
        Ok(_) => true,
        Err(_) => false,
    }
}
pub fn parse_command() -> Option<(String, String)> {
    print!("Enter command: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let trimmed_input = input.trim();

    if trimmed_input.starts_with(".file") || trimmed_input.starts_with(".image") {
        let parts: Vec<&str> = trimmed_input.splitn(2, ' ').collect();
        if parts.len() == 2 {
            return Some((parts[0].to_string(), parts[1].to_string()));
        } else {
            return Some((".text".to_string(), trimmed_input.to_string()));
        }
    } else {
        return Some((".text".to_string(), trimmed_input.to_string()));
    }
}