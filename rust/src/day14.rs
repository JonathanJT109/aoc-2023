use anyhow::Result;
use aoc::print_answers;
use std::collections::HashMap;

fn total_load(platform: &[Vec<char>]) -> usize {
    let mut sum = 0;
    for (x, row) in platform.iter().enumerate() {
        for col in row {
            if *col == 'O' {
                sum += platform.len() - x;
            }
        }
    }
    sum
}

fn rotate_counterclockwise(platform: &[Vec<char>]) -> Vec<Vec<char>> {
    let rows = platform.len();
    let cols = platform[0].len();
    let mut new_platform = vec![vec!['.'; cols]; rows];

    for (x, row) in new_platform.iter_mut().enumerate() {
        for y in 0..rows {
            row[y] = platform[rows - 1 - y][x];
        }
    }

    new_platform
}

fn tilt_north(platform: &mut [Vec<char>]) {
    for col in 0..platform[0].len() {
        for row in 0..platform.len() {
            let mut rock = row;
            while rock < platform.len() {
                if platform[rock][col] == '#' {
                    break;
                }
                rock += 1;
            }

            if platform[row][col] == '.' {
                for i in row..rock {
                    if platform[i][col] == 'O' {
                        platform[row][col] = 'O';
                        platform[i][col] = '.';
                        break;
                    }
                }
            }
        }
    }
}

fn part_1(lines: &[String]) -> usize {
    let mut lines = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    tilt_north(&mut lines);
    total_load(&lines)
}

fn cycle(platform: &mut Vec<Vec<char>>) {
    for _ in 0..4 {
        tilt_north(platform);
        *platform = rotate_counterclockwise(platform);
    }
}

fn part_2(lines: &[String]) -> usize {
    let mut lines = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let mut seen: HashMap<Vec<Vec<char>>, usize> = HashMap::new();

    for i in 0..1_000_000_000 {
        cycle(&mut lines);

        if let Some(before) = seen.get(&lines) {
            let size = i - before;
            let remaining = 1_000_000_000 - i - 1;
            let remaining = remaining % size;

            for _ in 0..remaining {
                cycle(&mut lines);
            }

            return total_load(&lines);
        } else {
            seen.insert(lines.clone(), i);
        }
    }

    0
}

fn main() -> Result<()> {
    print_answers(14, &part_1, &part_2);
    Ok(())
}
