use anyhow::Result;
use aoc::testing;

#[derive(PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

fn energized(tiles: &[Vec<char>]) -> usize {
    tiles
        .iter()
        .map(|file| {
            file.iter()
                .filter(|char| **char == '#')
                .collect::<Vec<_>>()
                .len()
        })
        .sum()
}

fn clockwise(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
        Direction::Right => Direction::Down,
        Direction::Left => Direction::Up,
    }
}
fn counterclockwise(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Left,
        Direction::Down => Direction::Right,
        Direction::Right => Direction::Up,
        Direction::Left => Direction::Down,
    }
}

fn next_coor(x: isize, y: isize, dir: Direction) -> (isize, isize, Direction) {
    match dir {
        Direction::Up => (x - 1, y, dir),
        Direction::Down => (x + 1, y, dir),
        Direction::Right => (x, y + 1, dir),
        Direction::Left => (x, y - 1, dir),
    }
}

fn beam(tiles: &mut [Vec<char>], x: isize, y: isize, dir: Direction) {
    if (x < 0 || x as usize >= tiles.len()) || (y < 0 || y as usize >= tiles.len()) {
        return;
    }

    let tile = &mut tiles[x as usize][y as usize];

    match *tile {
        '.' | '#' => {
            *tile = '#';
            let next = next_coor(x, y, dir);
            beam(tiles, next.0, next.1, next.2);
        }
        '|' => {
            if dir == Direction::Up || dir == Direction::Down {
                let next = next_coor(x, y, dir);
                beam(tiles, next.0, next.1, next.2);
            } else {
                beam(tiles, x - 1, y, Direction::Up);
                beam(tiles, x + 1, y, Direction::Down);
            }
        }
        '-' => {
            if dir == Direction::Left || dir == Direction::Right {
                let next = next_coor(x, y, dir);
                beam(tiles, next.0, next.1, next.2);
            }
            beam(tiles, x, y - 1, Direction::Left);
            beam(tiles, x, y + 1, Direction::Right);
        }
        '/' => {
            if dir == Direction::Up || dir == Direction::Down {
                let dir = clockwise(dir);
                let next = next_coor(x, y, dir);
                *tile = '#';
                beam(tiles, next.0, next.1, next.2);
            } else {
                let dir = counterclockwise(dir);
                let next = next_coor(x, y, dir);
                *tile = '#';
                beam(tiles, next.0, next.1, next.2);
            }
        }
        '\\' => {
            if dir == Direction::Up || dir == Direction::Down {
                let dir = counterclockwise(dir);
                let next = next_coor(x, y, dir);
                *tile = '#';
                beam(tiles, next.0, next.1, next.2);
            } else {
                let dir = clockwise(dir);
                let next = next_coor(x, y, dir);
                *tile = '#';
                beam(tiles, next.0, next.1, next.2);
            }
        }
        _ => (),
    }
}

fn part_1(lines: &[String]) -> usize {
    let mut lines = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    beam(&mut lines, 0, 0, Direction::Right);
    println!();
    for line in &lines {
        println!("{}", line.iter().collect::<String>());
    }

    energized(&lines)
}

fn main() -> Result<()> {
    testing(16, &part_1, 46);
    Ok(())
}
