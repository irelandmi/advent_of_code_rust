use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input_test.txt")?;
    let reader = BufReader::new(file);

    let mut empty_line_found = false;
    let mut book_vec: Vec<>

    for line_result in reader.lines() {
        let line = line_result?; // Read the line as a string

        if line.trim().is_empty() {
            println!("Rules Collected");
            empty_line_found = true;
            break;
        } else {
            println!("{}", line);
        }
    }

    if empty_line_found {
        println!("An empty line was found.");
    }

    println!("Program Complete");
    Ok(())
}