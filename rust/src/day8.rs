use anyhow::Result;
use aoc::print_answers;
use regex::Regex;
use std::collections::HashMap;
use std::mem;

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

fn gcd(a: &usize, b: &usize) -> usize {
    let mut max = *a;
    let mut min = *b;
    if min > max {
        mem::swap(&mut max, &mut min);
    }
    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }
        max = min;
        min = res;
    }
}

fn lcm(a: &usize, b: &usize) -> usize {
    a * b / gcd(a, b)
}

fn part_2(lines: &[String]) -> usize {
    let instructions = lines[0].as_str().chars().collect::<Vec<char>>();
    let re = Regex::new(r"(\w{3})").unwrap();
    let mut navigation: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut start: Vec<&str> = Vec::new();
    let mut answer: usize = 1;

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

    for i in &start {
        let a = n_steps(i, &navigation, &instructions);
        answer = lcm(&answer, &a);
    }

    answer
}

fn main() -> Result<()> {
    print_answers(8, &part_1, &part_2);
    Ok(())
}
