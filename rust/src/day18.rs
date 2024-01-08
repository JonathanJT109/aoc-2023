use anyhow::Result;
use aoc::print_answers;
use itertools::Itertools;

// Idea from HyperNeutrino
// Shoelace Formula: (1/2) sum(x_i (y_(i + 1) - y_(i - 1)))
//                   (1/2) sum(y_i (x_(i - 1) - y_(i + 1)))
// Pick's Theorem: A = i + (b / 2) - 1

fn parsing(lines: &[String]) -> Vec<(char, isize, String)> {
    lines
        .iter()
        .map(|line| {
            let line = line.trim().split(' ').collect::<Vec<_>>();
            let d = line[0].chars().next().unwrap();
            let s = line[1].parse::<isize>().unwrap();
            let c = line[2][2..line[2].len() - 1].to_string();
            (d, s, c)
        })
        .collect::<Vec<_>>()
}

fn area(grid: &[(isize, isize)], vertices: &usize) -> usize {
    let area: isize = grid
        .iter()
        .circular_tuple_windows::<(_, _, _)>()
        .map(|(a, b, c)| b.0 * (a.1 - c.1))
        .sum::<isize>()
        .abs();
    let i = (area as usize / 2) - (*vertices / 2) + 1;

    i + vertices
}

fn part_1(lines: &[String]) -> usize {
    let mut dx = [0_isize; 2];
    let mut vertices = 0;

    let plan: Vec<(isize, isize)> = parsing(lines)
        .iter()
        .map(|(d, s, _)| {
            match *d {
                'U' => dx[0] -= *s,
                'D' => dx[0] += *s,
                'L' => dx[1] -= *s,
                _ => dx[1] += *s,
            };
            vertices += *s as usize;
            (dx[0], dx[1])
        })
        .collect();

    area(&plan, &vertices)
}

fn part_2(lines: &[String]) -> usize {
    let mut dx = [0_isize; 2];
    let mut vertices = 0;

    let plan: Vec<(isize, isize)> = parsing(lines)
        .iter()
        .map(|(_, _, c)| {
            let c = c.chars().collect::<Vec<_>>();
            let s =
                isize::from_str_radix(&c.iter().take(c.len() - 1).collect::<String>(), 16).unwrap();
            match c[c.len() - 1].to_digit(10).unwrap() {
                3 => dx[0] -= s,
                1 => dx[0] += s,
                2 => dx[1] -= s,
                _ => dx[1] += s,
            };
            vertices += s as usize;
            (dx[0], dx[1])
        })
        .collect();

    area(&plan, &vertices)
}

fn main() -> Result<()> {
    print_answers(18, &part_1, &part_2);
    Ok(())
}
