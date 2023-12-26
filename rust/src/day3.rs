use std::usize;

use anyhow::Result;
use aoc::print_answers;

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

fn part_1(lines: &[String]) -> usize {
    let mut numbers = Vec::<usize>::new();
    let new_lines: Vec<Vec<char>> = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    for (x, line) in new_lines.iter().enumerate() {
        let mut check: bool = false;
        let mut number: usize = 0;
        for (y, letter) in line.iter().enumerate() {
            if let Some(digit) = letter.to_digit(10) {
                number = number * 10 + digit as usize;

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

    // for i in &numbers {
    //     println!("{i}");
    // }
    numbers.iter().sum()
}

#[derive(Debug)]
struct Gear {
    x: usize,
    y: usize,
    numbers: Vec<usize>,
}

fn is_symbol(
    lines: &Vec<Vec<char>>,
    x: &isize,
    y1: &usize,
    y2: &usize,
    number: &usize,
    gears: &mut Vec<Gear>,
) {
    let h = lines.len();
    let w = lines[0].len();

    if *x >= 0_isize && *x < h as isize {
        for i in (*y1 as isize - 1)..(*y2 as isize + 1) {
            if i >= 0 && i < w as isize && lines[*x as usize][i as usize] == '*' {
                if let Some(result) = gears
                    .iter_mut()
                    .find(|gear| gear.x == *x as usize && gear.y == i as usize)
                {
                    result.numbers.push(*number);
                } else {
                    gears.push(Gear {
                        x: *x as usize,
                        y: i as usize,
                        numbers: vec![*number],
                    });
                }
            }
        }
    }
}

fn part_2(lines: &[String]) -> usize {
    let mut gears = Vec::<Gear>::new();
    let new_lines: Vec<Vec<char>> = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let width = new_lines[0].len();

    for (x, line) in new_lines.iter().enumerate() {
        let mut y = 0;
        while y < width {
            let mut number: usize = 0;
            let start = y;

            while y < width && line[y].is_ascii_digit() {
                let digit: usize = line[y].to_digit(10).unwrap() as usize;
                number = number * 10 + digit;
                y += 1;
            }

            if number == 0 {
                y += 1;
                continue;
            }

            for idx in [-1, 0, 1].iter() {
                is_symbol(
                    &new_lines,
                    &(x as isize + idx),
                    &start,
                    &y,
                    &number,
                    &mut gears,
                );
            }

            y += 1;
        }
    }

    let mut answer: usize = 0;

    for gear in &gears {
        if gear.numbers.len() == 2 {
            answer += gear.numbers.iter().product::<usize>();
        }
    }

    answer
}

fn main() -> Result<()> {
    print_answers(3, &part_1, &part_2);

    Ok(())
}
