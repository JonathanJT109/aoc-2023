use anyhow::Result;
// use aoc::read_file;
use aoc::read_file_to_lines;
use regex::Regex;
use std::collections::HashSet;

fn part_1(lines: &Vec<Vec<char>>) -> u32 {
    let re = Regex::new(r"\d+").unwrap();
    let mut sum = 0;

    for line in lines {
        let mut my_num: HashSet<u32> = HashSet::new();
        let mut i = 0;

        let line = line
            .split(|letter| *letter == ':' || *letter == '|')
            .collect::<Vec<_>>()
            .iter()
            .map(|slice| slice.iter().collect::<String>())
            .collect::<Vec<String>>();

        for m in re.find_iter(line[1].as_str()) {
            let m = m.as_str().parse::<u32>().unwrap();
            my_num.insert(m);
        }

        for win_num in re.find_iter(line[2].as_str()) {
            let win_num = win_num.as_str().parse::<u32>().unwrap();
            if my_num.contains(&win_num) {
                i += 1;
            }
        }

        if i > 0 {
            sum += 2_u32.pow(i - 1);
        }
    }
    sum
}

fn part_2(lines: &Vec<Vec<char>>) -> u32 {
    let re = Regex::new(r"\d+").unwrap();
    let mut instances: Vec<u32> = vec![1; lines.len()];

    for (x, line) in lines.iter().enumerate() {
        let mut my_num: HashSet<u32> = HashSet::new();
        let mut i = 0;

        let line = line
            .split(|letter| *letter == ':' || *letter == '|')
            .collect::<Vec<_>>()
            .iter()
            .map(|slice| slice.iter().collect::<String>())
            .collect::<Vec<String>>();

        for m in re.find_iter(line[1].as_str()) {
            let m = m.as_str().parse::<u32>().unwrap();
            my_num.insert(m);
        }

        for win_num in re.find_iter(line[2].as_str()) {
            let win_num = win_num.as_str().parse::<u32>().unwrap();
            if my_num.contains(&win_num) {
                i += 1;
            }
        }
        // 1 2 3 4 4 1

        let mut j = 1;
        while i > 0 {
            if x > 0 {
                instances[x + j] += instances[x];
            } else {
                instances[x + j] += 1;
            }
            j += 1;
            i -= 1;
        }
    }

    instances.iter().sum()
}

fn main() -> Result<()> {
    if let Ok(file) = read_file_to_lines("./src/input/day4.txt") {
        let answer = part_1(&file);
        println!("Answer: {}", answer);
    } else {
        eprintln!("ERROR: File not found");
    }

    if let Ok(file) = read_file_to_lines("./src/input/day4.txt") {
        let answer = part_2(&file);
        println!("Answer: {}", answer);
    } else {
        eprintln!("ERROR: File not found");
    }

    Ok(())
}
