use anyhow::Result;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

pub fn print_answers<T: Debug>(
    day: usize,
    part_1: &dyn Fn(&[String]) -> T,
    part_2: &dyn Fn(&[String]) -> T,
) {
    let input_file: String = format!("./src/input/day{}.txt", day);
    let input_file: &str = input_file.as_str();
    if let Ok(file) = read_file(input_file) {
        let answer = part_1(&file);
        println!("Answer (part 1): {:?}", answer);

        let answer = part_2(&file);
        println!("Answer (part 2): {:?}", answer);
    } else {
        eprintln!("ERROR: File not found");
    }
}

pub fn testing<T: Debug + Eq>(day: usize, part_1: &dyn Fn(&[String]) -> T, solution: T) -> bool {
    let input_file: String = format!("./src/input/day{}.txt", day);
    let input_file: &str = input_file.as_str();
    if let Ok(file) = read_file(input_file) {
        let answer = part_1(&file);
        println!("Output: {:?}", answer);
        if answer == solution {
            println!("TEST PASSED");
            return true;
        }
    } else {
        eprintln!("ERROR: File not found");
    }
    eprintln!("TEST FAILED");
    false
}
