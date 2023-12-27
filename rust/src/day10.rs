use anyhow::Result;
use aoc::print_answers;

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

fn next(pipe: &char, direction: &Direction) -> (isize, isize, Direction) {
    match *pipe {
        '|' => {
            if *direction == Direction::Up {
                return (-1, 0, Direction::Up);
            }
            return (1, 0, Direction::Down);
        }
        '-' => {
            if *direction == Direction::Right {
                return (0, 1, Direction::Right);
            }
            return (0, -1, Direction::Left);
        }
        'L' => {
            if *direction == Direction::Down {
                return (0, 1, Direction::Right);
            }
            return (-1, 0, Direction::Up);
        }
        'J' => {
            if *direction == Direction::Down {
                return (0, -1, Direction::Left);
            }
            return (-1, 0, Direction::Up);
        }
        '7' => {
            if *direction == Direction::Up {
                return (0, -1, Direction::Left);
            }
            return (1, 0, Direction::Down);
        }
        'F' => {
            if *direction == Direction::Up {
                return (0, 1, Direction::Right);
            }
            return (1, 0, Direction::Down);
        }
        _ => return (0, 0, Direction::Up),
    }
}

fn steps(
    mapping: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<u8>>,
    x_pos: &usize,
    y_pos: &usize,
    direction: Direction,
) -> usize {
    let mut steps: usize = 1;
    let mut is_loop: bool = false;
    let mut x: isize = *x_pos as isize;
    let mut y: isize = *y_pos as isize;
    let mut dir = direction;
    visited[x as usize][y as usize] = 1;

    while is_loop == false && mapping[x as usize][y as usize] != '.' {
        let current = mapping[x as usize][y as usize];
        let next_pipe = next(&current, &dir);
        x += next_pipe.0;
        y += next_pipe.1;
        dir = next_pipe.2;
        steps += 1;
        visited[x as usize][y as usize] = 1;

        if current == 'S' {
            is_loop = true;
        }
    }

    if is_loop == true {
        return (steps / 2) as usize;
    }

    0
}

fn explore(lines: &Vec<Vec<char>>, start: &[usize; 2], visited: &mut Vec<Vec<u8>>) -> usize {
    let offsets: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let h = lines.len() as isize;
    let w = lines[0].len() as isize;
    let mut furthest: usize = 0;

    for (dx, dy) in &offsets {
        if start[0] as isize + *dx > 0
            && start[0] as isize + *dx <= h
            && start[1] as isize + *dy > 0
            && start[1] as isize + *dy <= w
        {
            let further: usize = match (*dx, *dy) {
                (1, 0) => steps(&lines, visited, &(start[0] + 1), &start[1], Direction::Down),
                (-1, 0) => steps(&lines, visited, &(start[0] - 1), &start[1], Direction::Up),
                (0, 1) => steps(
                    &lines,
                    visited,
                    &start[0],
                    &(start[1] + 1),
                    Direction::Right,
                ),
                (0, -1) => steps(&lines, visited, &start[0], &(start[1] - 1), Direction::Left),
                _ => 0,
            };

            furthest = furthest.max(further);
        }
    }
    furthest
}

fn part_1(lines: &[String]) -> usize {
    let mut start: [usize; 2] = [0; 2];
    let lines = lines
        .iter()
        .enumerate()
        .map(|(x, l)| {
            l.chars()
                .enumerate()
                .map(|(y, c)| {
                    if c == 'S' {
                        start = [x, y];
                    }
                    c
                })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<_>>();
    let mut visited: Vec<Vec<u8>> = vec![vec![0; lines[0].len()]; lines.len()];

    explore(&lines, &start, &mut visited)
}

// Ray-casting algorithm. Idea from William Y. Feng
fn inside(mapping: &Vec<Vec<char>>, x_pos: &usize, y_pos: &usize, visited: &Vec<Vec<u8>>) -> usize {
    let mut intersections: usize = 0;
    let pipes = ['J', 'L', '|'];

    for y in 0..*y_pos {
        if visited[*x_pos][y as usize] == 0 {
            continue;
        }
        if pipes.contains(&mapping[*x_pos][y as usize]) {
            intersections += 1;
        }
    }

    if intersections % 2 == 1 {
        return 1;
    }
    0
}

fn part_2(lines: &[String]) -> usize {
    let mut start = [0, 0];
    let lines = lines
        .iter()
        .enumerate()
        .map(|(x, l)| {
            l.chars()
                .enumerate()
                .map(|(y, c)| {
                    if c == 'S' {
                        start = [x, y];
                    }
                    c
                })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<_>>();

    let mut visited: Vec<Vec<u8>> = vec![vec![0; lines[0].len()]; lines.len()];
    let mut tiles: usize = 0;
    explore(&lines, &start, &mut visited);

    for x in 0..visited.len() {
        for y in 0..visited[0].len() {
            if visited[x][y] == 0 {
                tiles += inside(&lines, &x, &y, &visited);
            }
        }
    }

    tiles
}

fn main() -> Result<()> {
    print_answers(10, &part_1, &part_2);
    Ok(())
}
