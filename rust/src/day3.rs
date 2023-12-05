use anyhow::Result;
use aoc::read_file;

fn check_surr(lines: &Vec<Vec<char>>, x: &usize, y: &usize) -> bool {
    let offsets = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let x_len = lines.len() as isize;
    let y_len = lines[0].len() as isize;

    for &(dx, dy) in &offsets {
        let row = *x as isize + dx;
        let col = *y as isize + dy;

        if row >= 0 && row < x_len && col >= 0 && col < y_len {
            let adjacent = lines[row as usize][col as usize];
            if !adjacent.is_ascii_alphanumeric() && adjacent != '.' {
                return true;
            }
        }
    }

    false
}

fn part_1(lines: &[String]) -> u32 {
    let mut numbers = Vec::<u32>::new();
    let new_lines: Vec<Vec<char>> = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    for (x, line) in new_lines.iter().enumerate() {
        let mut check: bool = false;
        let mut number: u32 = 0;
        for (y, letter) in line.iter().enumerate() {
            if let Some(digit) = letter.to_digit(10) {
                number = number * 10 + digit;

                if !check {
                    check = check_surr(&new_lines, &x, &y);
                }
            }
            if y == line.len() - 1 || !(*letter).is_alphanumeric() {
                if number > 0 && check {
                    numbers.push(number);
                }
                number = 0;
                check = false;
            }
        }
    }

    for i in &numbers {
        println!("{i}");
    }
    // for line in &new_lines {
    //     for letter in line {
    //         print!("{letter}");
    //     }
    //     println!();
    // }

    numbers.iter().sum()
}

fn main() -> Result<()> {
    if let Ok(file) = read_file("../day3.txt") {
        let answer = part_1(&file);
        println!("Answer: {}", answer);
    } else {
        eprintln!("ERROR: File not found");
    }

    if let Ok(_file) = read_file("./src/input/day3.txt") {
        // let answer = part_2(&file);
        // println!("Answer: {}", answer);
    } else {
        eprintln!("ERROR: File not found");
    }

    Ok(())
}
