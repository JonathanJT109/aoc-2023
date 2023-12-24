use anyhow::Result;
use aoc::read_file;
use regex::Regex;

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

fn n_possible(race: &Race) -> usize {
    let mut i = 1;
    let mut j = race.time - 1;
    let mut found = [false; 2];

    println!("T: {}, D: {}", race.time, race.distance);
    while i < j && (found[0] == false || found[1] == false) {
        if found[0] == false {
            let i_dist = (race.time - i) * i;
            if i_dist > race.distance {
                found[0] = true;
                continue;
            }
            i += 1;
        }
        if found[1] == false {
            let j_dist = (race.time - j) * j;
            if j_dist > race.distance {
                found[1] = true;
                continue;
            }
            j -= 1;
        }
    }
    println!("{}, {}", i, j);

    j - i + 1
}

fn part_1(lines: &[String]) -> usize {
    let re = Regex::new(r"\d+").unwrap();
    let mut answer = 1;

    let times: Vec<usize> = re
        .find_iter(lines[0].as_str())
        .map(|num| num.as_str().parse::<usize>().unwrap())
        .collect();

    let distances: Vec<usize> = re
        .find_iter(lines[1].as_str())
        .map(|num| num.as_str().parse::<usize>().unwrap())
        .collect();

    for i in 0..times.len() {
        let race = Race {
            time: times[i],
            distance: distances[i],
        };
        answer *= n_possible(&race);
    }

    answer
}

fn part_2(lines: &[String]) -> usize {
    let re = Regex::new(r"\d+").unwrap();
    let mut answer = 1;

    let lines: Vec<String> = lines
        .iter()
        .map(|line| {
            line.chars()
                .filter(|letter| !letter.is_whitespace())
                .collect()
        })
        .collect();

    let times: Vec<usize> = re
        .find_iter(lines[0].as_str())
        .map(|num| num.as_str().parse::<usize>().unwrap())
        .collect();

    let distances: Vec<usize> = re
        .find_iter(lines[1].as_str().trim())
        .map(|num| num.as_str().parse::<usize>().unwrap())
        .collect();

    for i in 0..times.len() {
        let race = Race {
            time: times[i],
            distance: distances[i],
        };
        answer *= n_possible(&race);
    }

    answer
}

fn main() -> Result<()> {
    // if let Ok(file) = read_file("./src/input/day6.txt") {
    //     let answer = part_1(&file);
    //     println!("Answer: {}", answer);
    // } else {
    //     eprintln!("ERROR: File not found");
    // }

    if let Ok(file) = read_file("./src/input/day6.txt") {
        let answer = part_2(&file);
        println!("Answer: {}", answer);
    } else {
        eprintln!("ERROR: File not found");
    }

    Ok(())
}
