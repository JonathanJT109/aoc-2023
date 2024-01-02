use anyhow::Result;
use aoc::print_answers;

fn v_ref(pattern: &String, n: usize, prev: usize) -> usize {
    let pattern: Vec<Vec<char>> = pattern
        .split('\n')
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    for i in 0..pattern[0].len() - 1 {
        let mut is_ref: bool = true;
        let mut smudge = n;

        // println!("\n{i}");
        for row in &pattern {
            let mut col: usize = 0;
            while i as isize - col as isize >= 0 && i + col + 1 < pattern[0].len() {
                // println!("{} <-> {}", row[i - col], row[i + col + 1]);
                if row[i - col] != row[i + col + 1] {
                    if smudge > 0 {
                        smudge -= 1;
                        col += 1;
                        continue;
                    }
                    is_ref = false;
                    break;
                }
                col += 1;
            }
            if is_ref == false {
                break;
            }
        }

        if is_ref == true {
            if prev > 0 && prev - 1 == i {
                continue;
            }
            return i + 1;
        }
    }

    0
}

fn h_ref(pattern: &String, n: usize, prev: usize) -> usize {
    let pattern: Vec<Vec<char>> = pattern
        .split('\n')
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    for i in 0..pattern.len() - 1 {
        let mut is_ref: bool = true;
        let mut row: usize = 0;
        let mut smudge = n;

        while i as isize - row as isize >= 0 && i + row + 1 < pattern.len() {
            for col in 0..pattern[0].len() {
                if pattern[i - row][col] != pattern[i + row + 1][col] {
                    if smudge > 0 {
                        smudge -= 1;
                        continue;
                    }
                    is_ref = false;
                    break;
                }
            }
            // println!(
            //     "{i}: {} <-> {}",
            //     pattern[i - row].iter().collect::<String>(),
            //     pattern[i + row + 1].iter().collect::<String>()
            // );
            if is_ref == false {
                break;
            }
            row += 1;
        }

        if is_ref == true {
            if prev > 0 && prev - 1 == i {
                continue;
            }
            return i + 1;
        }
    }

    0
}

fn part_1(lines: &[String]) -> usize {
    let patterns: Vec<String> = lines
        .join("\n")
        .split("\n\n")
        .map(|m| m.to_string())
        .collect();
    let mut sum: usize = 0;

    for pattern in &patterns {
        let mut reflection = v_ref(pattern, 0, 0);
        if reflection == 0 {
            reflection = 100 * h_ref(pattern, 0, 0);
        }
        sum += reflection;
    }

    sum
}

fn part_2(lines: &[String]) -> usize {
    let patterns: Vec<String> = lines
        .join("\n")
        .split("\n\n")
        .map(|m| m.to_string())
        .collect();
    let mut sum: usize = 0;

    for pattern in &patterns {
        let prev_ref = (v_ref(pattern, 0, 0), h_ref(pattern, 0, 0));
        let mut reflection = v_ref(pattern, 1, prev_ref.0);
        if reflection == 0 {
            reflection = 100 * h_ref(pattern, 1, prev_ref.1);
        }
        sum += reflection;
    }

    sum
}

fn main() -> Result<()> {
    print_answers(13, &part_1, &part_2);
    Ok(())
}
