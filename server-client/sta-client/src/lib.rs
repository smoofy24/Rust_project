//! This crate handles the client side actions for server-client app
//!
//! # File handling functions:
//!
//! dir_exists(path: &str) -> io::Result<bool>
//! is_writable(path: &str) -> io::Result<bool>
//! create_dir(path: &str) -> io::Result<()>
//! is_valid_file(path: &str) -> io::Result<bool>
//!
//! # Client handling functions:
//!
//! strip_to_second_space(cow: Cow<str>) -> Cow<str>
//! parse_command() -> Option<(String, String)>

use std::io::{self, Write};
use std::fs;
use std::path::Path;
use std::borrow::Cow;


/// Checks if the folder exists in destination
fn dir_exists(path: &str) -> io::Result<bool> {
    Ok(fs::metadata(path).map(|meta| meta.is_dir()).unwrap_or(false))
}

/// Checks if the PATH is writable
fn is_writable(path: &str) -> io::Result<bool> {
    fs::OpenOptions::new()
        .write(true)
        .create(false)
        .open(path)
        .map(|_| true)
        .or_else(|e| match e.kind() {
            io::ErrorKind::PermissionDenied => Ok(false),
            _ => Err(e),
        })
}

/// Creates directory
pub fn create_dir(path: &str) -> io::Result<()> {
    if dir_exists(path)? {
        if !is_writable(path)? {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                format!("Directory '{}' is not writable!", path),
            ));
        }
    } else {
        // Directory doesn't exist, attempt to create it
        return fs::create_dir_all(path);
    }

    // Directory exists and is writable, return Ok(())
    Ok(())
}

/// Check is PATH is a valid file and is readable
pub fn is_valid_file(path: &str) -> io::Result<(bool)> {
    let path = Path::new(path);

    /// Check if the path exists and is a file
    if !path.exists() || !path.is_file() {
        return  Ok(false);
    }

    // Check if the file is readable
    match fs::File::open(path) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

/// Strips the data into 2 parts
pub fn strip_to_second_space(cow: Cow<str>) -> Cow<str> {
    let s: &str = &cow;
    let mut spaces = s.match_indices(' ').take(2).map(|(index, _)| index);

    if let Some(second_space) = spaces.nth(1) {
        Cow::Owned(s[second_space + 1..].to_string())
    } else {
        cow // If there are less than two spaces, return the original Cow
    }
}

/// Takes input and parses entered command into two parts COMMAND and TEXT
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