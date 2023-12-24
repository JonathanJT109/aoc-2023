use anyhow::Result;
use aoc::read_file;

fn ranking(hands: &[String]) -> Vec<usize> {
    vec![1; hands.len()]
}

fn part_1(lines: &[String]) -> usize {
    let mut answer: usize = 0;
    let hands: Vec<String> = lines
        .iter()
        .map(|line| line.split(' ').nth(0).unwrap().to_string())
        .collect();

    let bids: Vec<usize> = lines
        .iter()
        .map(|line| line.split(' ').nth(1).unwrap().parse::<usize>().unwrap())
        .collect();

    let ranks = ranking(&hands);

    for i in 0..hands.len() {
        println!("{}", ranks[i] * bids[i]);
        answer += ranks[i] * bids[i];
    }

    answer
}

fn main() -> Result<()> {
    if let Ok(file) = read_file("./src/input/day7.txt") {
        let answer = part_1(&file);
        println!("{answer}");
    }

    Ok(())
}
