mod modules;

use clap::Parser;
use slug::slugify;
use modules::my_funcs::*;

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
    ///CSV parse
    #[arg(long, action)]
    csv: bool,
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
        options.csv,
    ]
        .iter()
        .filter(|&flag| *flag)
        .count();

    if opt_num != 1 {
        println!("Exactly one argument must be provided!");
        return;
    }

    let transformed_output = match options {
        Args { lowercase: true, .. } => lowercase(&read_input()),
        Args { uppercase: true, .. } => uppercase(&read_input()),
        Args { no_spaces: true, .. } => no_spaces(&read_input()),
        Args { slugify: true, .. } => slugify(&read_input()),
        Args { reverse: true, .. } => reverse(&read_input()),
        Args { switch_case: true, .. } => switchcase(&read_input()),
        Args { csv: true, .. } => {
            match csv_parser(&read_csv()) {
                Ok(records) => format_as_table(records),
                Err(err) => {
                    println!("Error parsing CSV: {}", err);
                    return;
                }
            }
        },
        _ => {
            println!("Invalid argument!!!");
            return;
        }
    };

    println!("{}", transformed_output);
}