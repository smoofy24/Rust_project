mod modules;

use clap::{command, Parser};
use slug::slugify;
use modules::my_funcs::*;
use std::path::Path;
use std::{fs, thread};
use std::sync::mpsc;


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
    /// CSV path (optional)
    #[arg(long, value_name = "PATH")]
    csv_path: Option<String>,
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

    if opt_num == 1 {
        let transformed_output = match options {
            Args { lowercase: true, .. } => lowercase(&read_input(true)),
            Args { uppercase: true, .. } => uppercase(&read_input(true)),
            Args { no_spaces: true, .. } => no_spaces(&read_input(true)),
            Args { slugify: true, .. } => slugify(&read_input(true)),
            Args { reverse: true, .. } => reverse(&read_input(true)),
            Args { switch_case: true, .. } => switchcase(&read_input(true)),
            Args { csv: true, .. } => {

                if let Some(ref path_str) = options.csv_path {
                    let csv_path = Path::new(path_str);
                        match fs::metadata(csv_path) {
                            Err(e) => {
                                eprintln!("Error reading the file {:?}: {}", csv_path, e);
                                return;
                            }
                            Ok(path) => {
                                if path.is_file() {
                                    match read_csv_file(csv_path) {
                                        Some(csv_data) => {
                                            match csv_parser(&csv_data) {
                                                Ok(records) => {
                                                    format_as_table(records)
                                                }
                                                Err(err) => {
                                                    eprintln!("Error parsing CSV: {}", err);
                                                    return;
                                                }
                                            }
                                        }
                                        None => {
                                            eprintln!("Error getting CSV!");
                                            return;
                                        }
                                    }
                                } else {
                                    eprintln!("Provided path is not a file {:?}", path);
                                    return;
                                }
                            }
                        }
                } else {
                    match read_csv() {
                        Some(csv_data) => {
                            match csv_parser(&csv_data) {
                                Ok(records) => {
                                    format_as_table(records)
                                }
                                Err(err) => {
                                    eprintln!("Error parsing CSV: {}", err);
                                    return;
                                }
                            }
                        }
                        None => {
                            eprintln!("Error getting CSV!");
                            return;
                        }
                    }
                }
            },
            _ => {
                eprintln!("Invalid argument!!!");
                return;
            }
        };

        println!("{}", transformed_output);
    } else if opt_num == 0 {

        println!("Enter commands interactively:");
        let (tx, rx) = mpsc::channel();

        let reading_thread = thread::spawn( move || {

            loop {
                let input = read_input(false);

                if tx.send(input).is_err() {
                    eprintln!("Error sending input!!!");
                    break;
                };
            }

        });

        let processing_thread = thread::spawn( move || {

            loop {
                match rx.recv() {
                    Ok(received) => {

                        if let Some((operation, rest)) = separate_command(&received) {
                            let transformed_output = match operation {
                                "lowercase" => lowercase(&rest),
                                "uppercase" => uppercase(&rest),
                                "no_spaces" => no_spaces(&rest),
                                "slugify" => slugify(&rest),
                                "reverse" => reverse(&rest),
                                "switchcase" => switchcase(&rest),
                                "csv" => {
                                    match csv_parser(&rest) {
                                        Ok(records) => {
                                            format_as_table(records)
                                        }
                                        Err(err) => {
                                            eprintln!("Error parsing CSV: {}", err);
                                            return;
                                        }
                                    }
                                },
                                _ => {
                                    eprintln!("Wrong argument!!");
                                    break;
                                }
                            };
                            println!("{}", transformed_output);
                        }

                    },
                    Err(_) => {
                        println!("Channel closed");
                        break;
                    },
                }
            }
        });

        reading_thread.join().unwrap();
        processing_thread.join().unwrap();

    } else {
        eprintln!("Too many parameters!!!");
        return;
    }
}