use anyhow::Result;
use aoc::print_answers;
use regex::Regex;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Clone)]
struct Map {
    destination: usize,
    source: usize,
    range: usize,
}

fn mapping(map_list: &Vec<Map>, seeds: &mut Vec<usize>) {
    for seed in seeds {
        for m in map_list {
            if *seed >= m.source && *seed <= (m.source + m.range) {
                let diff: isize = *seed as isize + m.destination as isize - m.source as isize;
                *seed = diff as usize;
                break;
            }
        }
    }
}

fn part_1(lines: &[String]) -> usize {
    let re = Regex::new(r"\d+").unwrap();
    let pattern = Regex::new(r"\d+(?:\s+\d+)*").unwrap();
    let mut seeds = re
        .find_iter(lines[0].as_str())
        .map(|m| m.as_str().parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    // Iterate over regex captures in the text
    for capture in pattern.find_iter(lines[1..].join("\n").as_str()) {
        let mut map_list: Vec<Map> = Vec::new();
        let capture = capture.as_str();

        for line in capture.split('\n') {
            let line = line
                .split(' ')
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            map_list.push(Map {
                destination: line[0],
                source: line[1],
                range: line[2],
            });
        }

        mapping(&map_list, &mut seeds);
    }

    match seeds.iter().min() {
        Some(answer) => *answer,
        _ => 0,
    }
}

fn mapping_threads(start: &usize, end: &usize, rules: &Vec<Vec<Map>>) -> usize {
    let mut min_num = usize::MAX;
    for i in *start..=(*start + *end) {
        let mut seed = i;

        for rule in rules {
            for m in rule {
                if seed >= m.source && seed <= (m.source + m.range) {
                    let diff: isize = seed as isize + m.destination as isize - m.source as isize;
                    seed = diff as usize;
                    break;
                }
            }
        }

        min_num = min_num.min(seed);
    }

    println!("Start: {} | Range {}: Finished", start, end);
    min_num
}

fn part_2(lines: &[String]) -> usize {
    let re = Regex::new(r"\d+").unwrap();
    let pattern = Regex::new(r"\d+(?:\s+\d+)*").unwrap();
    let mut map_rules: Vec<Vec<Map>> = Vec::new();
    let seeds = re
        .find_iter(lines[0].as_str())
        .map(|m| m.as_str().parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    println!("-- Basics Done");

    for capture in pattern.find_iter(lines[1..].join("\n").as_str()) {
        let capture = capture.as_str();
        let mut map_section: Vec<Map> = Vec::new();

        for line in capture.split('\n') {
            let line: Vec<_> = line
                .split(' ')
                .map(|num| num.parse::<usize>().unwrap())
                .collect();

            map_section.push(Map {
                destination: line[0],
                source: line[1],
                range: line[2],
            });
        }
        map_rules.push(map_section);
    }
    println!("-- Mapping rules done");

    let mut i = 0;
    let mut handles = vec![];
    let seeds_c = Arc::new(seeds.clone());
    let rules_c = Arc::new(map_rules.clone());
    let mutex = Arc::new(Mutex::new(usize::MAX));

    while i < seeds.len() {
        let start = seeds_c[i];
        let range = seeds_c[i + 1];
        let rules_c = Arc::clone(&rules_c);
        let c_mutex = Arc::clone(&mutex);
        handles.push(thread::spawn(move || {
            let min_value = mapping_threads(&start, &range, &rules_c);
            let mut guard = c_mutex.lock().unwrap();
            *guard = guard.min(min_value);
        }));
        i += 2;
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let answer: usize = *mutex.lock().unwrap();

    answer
}

fn main() -> Result<()> {
    print_answers(5, &part_1, &part_2);

    Ok(())
}
