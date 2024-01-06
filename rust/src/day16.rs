use anyhow::Result;
use aoc::print_answers;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn clockwise(&mut self) {
        *self = match *self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Right => Direction::Down,
            Direction::Left => Direction::Up,
        }
    }
    fn counterclockwise(&mut self) {
        *self = match *self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
            Direction::Left => Direction::Down,
        }
    }
}

fn clear_visited(visited: &mut [Vec<char>]) {
    for l in visited {
        for c in l {
            *c = '.';
        }
    }
}

fn energized(tiles: &mut [Vec<char>]) -> usize {
    let sum = tiles
        .iter()
        .map(|file| {
            file.iter()
                .filter(|char| **char == '#')
                .collect::<Vec<_>>()
                .len()
        })
        .sum();
    clear_visited(tiles);
    sum
}

fn next_coor(x: &mut isize, y: &mut isize, dir: &mut Direction) {
    (*x, *y, *dir) = match dir {
        Direction::Up => (*x - 1, *y, *dir),
        Direction::Down => (*x + 1, *y, *dir),
        Direction::Right => (*x, *y + 1, *dir),
        Direction::Left => (*x, *y - 1, *dir),
    }
}

fn beam(
    tiles: &[Vec<char>],
    x: &mut isize,
    y: &mut isize,
    dir: &mut Direction,
    visited: &mut [Vec<char>],
    seen: &mut HashSet<(usize, usize, Direction)>,
) {
    while (*x >= 0 && (*x as usize) < tiles.len()) && (*y >= 0 && (*y as usize) < tiles[0].len()) {
        match tiles[*x as usize][*y as usize] {
            '|' => {
                if let None = seen.get(&(*x as usize, *y as usize, *dir)) {
                    seen.insert((*x as usize, *y as usize, *dir));
                } else {
                    return;
                }
                if *dir == Direction::Left || *dir == Direction::Right {
                    visited[*x as usize][*y as usize] = '#';
                    beam(
                        tiles,
                        &mut (*x - 1),
                        &mut (*y).clone(),
                        &mut Direction::Up,
                        visited,
                        seen,
                    );
                    beam(
                        tiles,
                        &mut (*x + 1),
                        &mut (*y).clone(),
                        &mut Direction::Down,
                        visited,
                        seen,
                    );
                    return;
                }
            }
            '-' => {
                if let None = seen.get(&(*x as usize, *y as usize, *dir)) {
                    seen.insert((*x as usize, *y as usize, *dir));
                } else {
                    return;
                }
                if *dir == Direction::Up || *dir == Direction::Down {
                    visited[*x as usize][*y as usize] = '#';
                    beam(
                        tiles,
                        &mut (*x).clone(),
                        &mut (*y + 1),
                        &mut Direction::Right,
                        visited,
                        seen,
                    );
                    beam(
                        tiles,
                        &mut (*x).clone(),
                        &mut (*y - 1),
                        &mut Direction::Left,
                        visited,
                        seen,
                    );
                    return;
                }
            }
            '/' => {
                if *dir == Direction::Up || *dir == Direction::Down {
                    (*dir).clockwise();
                } else {
                    (*dir).counterclockwise();
                }
            }
            '\\' => {
                if *dir == Direction::Up || *dir == Direction::Down {
                    (*dir).counterclockwise();
                } else {
                    (*dir).clockwise();
                }
            }
            _ => (),
        }
        visited[*x as usize][*y as usize] = '#';
        next_coor(x, y, dir);
    }
}

fn part_1(lines: &[String]) -> usize {
    let mut lines = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut visited = vec![vec!['.'; lines.len()]; lines[0].len()];
    let mut seen: HashSet<(usize, usize, Direction)> = HashSet::new();

    beam(
        &mut lines,
        &mut 0,
        &mut 0,
        &mut Direction::Right,
        &mut visited,
        &mut seen,
    );

    energized(&mut visited)
}

fn part_2(lines: &[String]) -> usize {
    let lines = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut visited = vec![vec!['.'; lines.len()]; lines[0].len()];
    let mut max_n: usize = 0;

    for mut x in 0..lines.len() as isize {
        let mut seen: HashSet<(usize, usize, Direction)> = HashSet::new();
        beam(
            &lines,
            &mut x,
            &mut 0,
            &mut Direction::Right,
            &mut visited,
            &mut seen,
        );
        max_n = max_n.max(energized(&mut visited));
        seen.clear();

        beam(
            &lines,
            &mut x,
            &mut (lines[0].len() as isize - 1),
            &mut Direction::Left,
            &mut visited,
            &mut seen,
        );
        max_n = max_n.max(energized(&mut visited));
    }

    for mut y in 0..lines[0].len() as isize {
        let mut seen: HashSet<(usize, usize, Direction)> = HashSet::new();
        beam(
            &lines,
            &mut 0,
            &mut y,
            &mut Direction::Down,
            &mut visited,
            &mut seen,
        );
        max_n = max_n.max(energized(&mut visited));
        seen.clear();

        beam(
            &lines,
            &mut (lines.len() as isize - 1),
            &mut y,
            &mut Direction::Up,
            &mut visited,
            &mut seen,
        );
        max_n = max_n.max(energized(&mut visited));
    }
    max_n
}

fn main() -> Result<()> {
    print_answers(16, &part_1, &part_2);
    Ok(())
}
