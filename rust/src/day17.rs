use anyhow::Result;
use aoc::testing;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Point {
    x: usize,
    y: usize,
}

fn part_1(lines: &[String]) -> usize {
    let lines = lines
        .iter()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for line in &lines {
        println!("{:?}", line);
    }

    0
}

fn main() -> Result<()> {
    testing(17, &part_1, 102);
    Ok(())
}
