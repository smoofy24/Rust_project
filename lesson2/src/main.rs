use std::io;
use std::io::Write;
use slug::slugify;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Exactly one argument must be provided!!");
        return;
    }

    let mut input = String::new();
    print!("Please enter the string: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read string!!");

    let transformed_output= match &args[1][..] {
        "lowercase" => input.to_lowercase(),
        "uppercase" => input.to_uppercase(),
        "no-spaces" => input.replace(" ",""),
        "slugify" => slugify(&input),
        "reverse" => input.chars().rev().collect(),
        "switch-case" => input.chars().map(|c| {
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
    println!("{transformed_output}");
}