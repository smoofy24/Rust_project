use std::io;
use std::io::Write;

fn main() {
    let mut name = String::new();
    print!("Enter your name please: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).expect("Failed to read string");
    let name = name.trim();
    print!("Hello, {name}!");
}
