use anyhow::Result;
use aoc::read_file;

fn part_1(lines: &[String]) -> u32 {
    let mut card_points = Vec::<u32>::new();

    for (row, line) in &lines.iter().enumerate() {}

    card_points.iter().sum()
}

fn main() -> Result<()> {
    if let Ok(file) = read_file("./src/input/day3.txt") {
        let answer = part_1(&file);
        println!("Answer: {}", answer);
    } else {
        eprintln!("ERROR: File not found");
    }

    if let Ok(file) = read_file("./src/input/day3.txt") {
        // let answer = part_2(&file);
        // println!("Answer: {}", answer);
    } else {
        eprintln!("ERROR: File not found");
    }

    Ok(())
}
