use anyhow::Result;
use aoc::print_answers;

fn distance(
    p1: (usize, usize),
    p2: (usize, usize),
    mt_rows: &Vec<bool>,
    mt_col: &Vec<bool>,
    n: usize,
) -> usize {
    let mut distance: usize = 0;

    let (x1, y1) = (p1.0.min(p2.0), p1.1.min(p2.1));
    let (x2, y2) = (p1.0.max(p2.0), p1.1.max(p2.1));

    for x in x1..x2 {
        distance += 1;
        if mt_rows[x] == true {
            distance += n - 1;
        }
    }

    for y in y1..y2 {
        distance += 1;
        if mt_col[y] == true {
            distance += n - 1;
        }
    }

    distance
}

fn part_1(lines: &[String]) -> usize {
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    let lines: Vec<Vec<char>> = lines
        .iter()
        .enumerate()
        .map(|(x, l)| {
            l.chars()
                .enumerate()
                .map(|(y, c)| {
                    if c == '#' {
                        galaxies.push((x, y));
                    }
                    c
                })
                .collect::<Vec<char>>()
        })
        .collect();

    let mut mt_rows: Vec<bool> = vec![false; lines.len()];
    let mut mt_col: Vec<bool> = vec![false; lines[0].len()];

    for (x, line) in lines.iter().enumerate() {
        if !line.contains(&'#') {
            mt_rows[x] = true;
        }
    }

    for y in 0..lines[0].len() {
        let mut mt: bool = true;
        for x in 0..lines.len() {
            if lines[x][y] == '#' {
                mt = false;
                break;
            }
        }
        if mt == true {
            mt_col[y] = true;
        }
    }

    let mut all_distances: usize = 0;
    for x in 0..galaxies.len() {
        for y in (x + 1)..galaxies.len() {
            all_distances += distance(galaxies[x], galaxies[y], &mt_rows, &mt_col, 1);
        }
    }

    all_distances
}

fn part_2(lines: &[String]) -> usize {
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    let lines: Vec<Vec<char>> = lines
        .iter()
        .enumerate()
        .map(|(x, l)| {
            l.chars()
                .enumerate()
                .map(|(y, c)| {
                    if c == '#' {
                        galaxies.push((x, y));
                    }
                    c
                })
                .collect::<Vec<char>>()
        })
        .collect();

    let mut mt_rows: Vec<bool> = vec![false; lines.len()];
    let mut mt_col: Vec<bool> = vec![false; lines[0].len()];

    for (x, line) in lines.iter().enumerate() {
        if !line.contains(&'#') {
            mt_rows[x] = true;
        }
    }

    for y in 0..lines[0].len() {
        let mut mt: bool = true;
        for x in 0..lines.len() {
            if lines[x][y] == '#' {
                mt = false;
                break;
            }
        }
        if mt == true {
            mt_col[y] = true;
        }
    }

    let mut all_distances: usize = 0;
    for x in 0..galaxies.len() {
        for y in (x + 1)..galaxies.len() {
            all_distances += distance(galaxies[x], galaxies[y], &mt_rows, &mt_col, 1000000);
        }
    }

    all_distances
}

fn main() -> Result<()> {
    print_answers(11, &part_1, &part_2);
    Ok(())
}
