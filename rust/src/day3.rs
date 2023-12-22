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

    // for i in &numbers {
    //     println!("{i}");
    // }
    numbers.iter().sum()
}

#[derive(Debug)]
struct Gear {
    x: usize,
    y: usize,
    numbers: Vec<u32>,
}

fn is_symbol(
    lines: &Vec<Vec<char>>,
    x: &isize,
    y1: &usize,
    y2: &usize,
    number: &u32,
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

fn part_2(lines: &[String]) -> u32 {
    let mut gears = Vec::<Gear>::new();
    let new_lines: Vec<Vec<char>> = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let width = new_lines[0].len();

    for (x, line) in new_lines.iter().enumerate() {
        let mut y = 0;
        while y < width {
            let mut number = 0;
            let start = y;

            while y < width && line[y].is_ascii_digit() {
                let digit = line[y].to_digit(10).unwrap();
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

    let mut answer: u32 = 0;

    for gear in &gears {
        if gear.numbers.len() == 2 {
            answer += gear.numbers.iter().product::<u32>();
        }
    }

    answer
}

fn main() -> Result<()> {
    if let Ok(file) = read_file("./src/input/day3.txt") {
        let answer = part_1(&file);
        println!("Answer: {}", answer);
    } else {
        eprintln!("ERROR: File not found");
    }

    if let Ok(file) = read_file("./src/input/day3.txt") {
        let answer = part_2(&file);
        println!("Answer: {}", answer);
    } else {
        eprintln!("ERROR: File not found");
    }

    Ok(())
}
