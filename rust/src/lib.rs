use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

// TODO: Create a function that takes a file and test it using two functions (part 1 and 2) and
// prints the result for both.

// Read the file
pub fn read_file(file_path: &str) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut information = Vec::<String>::new();

    for line in reader.lines() {
        match line {
            Ok(line) => information.push(line.to_string()),
            Err(err) => return Err(err),
        }
    }

    Ok(information)
}

pub fn read_file_to_lines(file_path: &str) -> Result<Vec<Vec<char>>, std::io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut information = Vec::<String>::new();

    for line in reader.lines() {
        match line {
            Ok(line) => information.push(line.to_string()),
            Err(err) => return Err(err),
        }
    }

    let information: Vec<Vec<char>> = information
        .iter()
        .map(|line| line.chars().collect())
        .collect();

    Ok(information)
}
