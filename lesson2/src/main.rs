use std::io;
use std::io::Write;
use slug::slugify;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Switch to lowercase
    #[arg(long, action)]
    lowercase: bool,
    /// Switch to uppercase
    #[arg(long, action)]
    uppercase: bool,
    /// Remove spaces
    #[arg(long, action)]
    no_spaces: bool,
    /// Slugify string
    #[arg(long, action)]
    slugify: bool,
    /// Reverse string
    #[arg(long, action)]
    reverse: bool,
    ///Switch case
    #[arg(long, action)]
    switch_case: bool,
}
fn main() {

    let options = Args::parse();

    let opt_num = [
        options.reverse,
        options.no_spaces,
        options.lowercase,
        options.switch_case,
        options.slugify,
        options.uppercase,
    ]
        .iter()
        .filter(|&flag| *flag)
        .count();

    if opt_num != 1 {
        println!("Exactly one argument must be provided!");
        return;
    }

    let mut input = String::new();
    print!("Please enter the string: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read string!!");

    let transformed_output = match options {
        Args { lowercase: true, .. } => input.to_lowercase(),
        Args { uppercase: true, .. } => input.to_uppercase(),
        Args { no_spaces: true, .. } => input.replace(" ", ""),
        Args { slugify: true, .. } => slugify(&input),
        Args { reverse: true, .. } => input.chars().rev().collect(),
        Args { switch_case: true, .. } => input.chars().map(|c| {
            if c.is_ascii_uppercase() {
                c.to_ascii_lowercase()
            } else if c.is_ascii_lowercase() {
                c.to_ascii_uppercase()
            } else {
                c
            }
        }).collect(),
        _ => {
            println!("Invalid argument!!!");
            return;
        }
    };

    println!("{}", transformed_output.trim());
}