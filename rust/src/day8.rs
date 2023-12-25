use anyhow::Result;
use aoc::read_file;
use regex::Regex;
use std::collections::HashMap;

fn part_1(lines: &[String]) -> usize {
    let instructions = lines[0].as_str().chars().collect::<Vec<char>>();
    let re = Regex::new(r"(\w{3})").unwrap();
    let mut navigation: HashMap<&str, (&str, &str)> = HashMap::new();

    for line in &lines[2..] {
        let line = re
            .find_iter(line)
            .map(|m| m.as_str())
            .collect::<Vec<&str>>();

        navigation.insert(line[0], (line[1], line[2]));
    }

    let mut i = 0;
    let mut steps = 0;
    let mut current: &str = "AAA";

    while current != "ZZZ" {
        let step: char = instructions[i];
        let next = navigation.get(current).unwrap();
        if step == 'L' {
            current = next.0;
        } else {
            current = next.1;
        }
        i = (i + 1) % instructions.len();
        steps += 1;
    }

    steps
}

fn n_steps(
    current: &str,
    navigation: &HashMap<&str, (&str, &str)>,
    instructions: &Vec<char>,
) -> usize {
    let mut i = 0;
    let mut steps = 0;
    let mut current = current.clone();

    loop {
        let step: char = instructions[i];

        let next = navigation.get(current).unwrap().clone();

        if step == 'L' {
            current = next.0;
        } else {
            current = next.1;
        }

        steps += 1;

        if current.chars().collect::<Vec<char>>()[2] == 'Z' {
            break;
        }

        i = (i + 1) % instructions.len();
    }

    steps
}

fn part_2(lines: &[String]) -> usize {
    let instructions = lines[0].as_str().chars().collect::<Vec<char>>();
    let re = Regex::new(r"(\w{3})").unwrap();
    let mut navigation: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut start: Vec<&str> = Vec::new();

    for line in &lines[2..] {
        let line = re
            .find_iter(line)
            .map(|m| m.as_str())
            .collect::<Vec<&str>>();

        let a: Vec<char> = line[0].chars().collect();

        navigation.insert(line[0], (line[1], line[2]));

        if a[2] == 'A' {
            start.push(line[0]);
        }
    }

    println!("{:?}", start);
    n_steps(&start[1], &navigation, &instructions)
}

fn main() -> Result<()> {
    // if let Ok(file) = read_file("./src/input/day8.txt") {
    //     let answer = part_1(&file);
    //     println!("Answer: {}", answer);
    // } else {
    //     eprintln!("ERROR: File not found");
    // }

    if let Ok(file) = read_file("./src/input/day8.txt") {
        let answer = part_2(&file);
        println!("Answer: {}", answer);
    } else {
        eprintln!("ERROR: File not found");
    }
    Ok(())
}
