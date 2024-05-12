use csv::{ReaderBuilder, StringRecord};
use prettytable::{Table, Row, Cell};
use std::fmt::Write as FmtWrite;
use std::io::{BufRead, self, Write};


pub fn csv_parser(csv_string: &str) -> Result<Vec<Vec<String>>, csv::Error> {
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(csv_string.as_bytes());
    let mut records = Vec::new();

    for result in reader.records() {
        let record: StringRecord = result?;
        let row = record.iter().map(|f| f.to_string()).collect();
        records.push(row);
    }
    Ok(records)
}

pub fn format_as_table(records: Vec<Vec<String>>) -> String {
    let mut table = Table::new();

    // Add data rows
    for row in records.iter() {
        let data_cells = row.iter().map(|cell| Cell::new(cell));
        table.add_row(Row::new(data_cells.collect()));
    }

    // Write table to a String
    let mut output = String::new();
    table.set_format(*prettytable::format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    write!(output, "{}", table).unwrap();

    output
}

pub fn read_input() -> String {
    let mut input = String::new();
    print!("Please enter the string: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read string!!");
    input
}

pub fn read_csv() -> String {
    let mut lines = String::new();
    let stdin = io::stdin();

    println!("Enter multiple lines of text (empty line to stop):");

    for line in stdin.lock().lines() {
        let line = line.expect("Failed to read line");
        if line.trim().is_empty() {
            break; // Stop reading lines if an empty line is encountered
        }
        lines.push_str(&line);
        lines.push('\n'); // Add a newline character after each line
    }

    lines
}

pub fn lowercase(string: &str) -> String {
    string.to_lowercase()
}

pub fn uppercase(string: &str) -> String {
    string.to_uppercase()
}

pub fn reverse(string: &str) -> String {
    string.chars().rev().collect()
}

pub fn no_spaces(string: &str) -> String {
    string.replace(" ", "")
}

pub fn switchcase(string: &str) -> String {
    string.chars().map(|c| {
        if c.is_ascii_uppercase() {
            c.to_ascii_lowercase()
        } else if c.is_ascii_lowercase() {
            c.to_ascii_uppercase()
        } else {
            c
        }
    }).collect()
}
