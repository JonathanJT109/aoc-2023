use anyhow::Result;
use aoc::testing;

fn n_arr(spring: &str, arr: &[usize]) -> usize {
    let new_spring = spring.chars().collect::<Vec<char>>();

    if arr.is_empty() {
        if new_spring.contains(&'#') {
            return 0;
        }
        return 1;
    }

    if new_spring.is_empty() {
        if arr.is_empty() {
            return 1;
        }
        return 0;
    }

    if new_spring.len() < arr.iter().sum::<usize>() + arr.len() - 1 {
        return 0;
    }

    let mut result: usize = 0;
    let n = arr[0];
    let end: usize = (n + 1).min(spring.len());

    if new_spring[0] == '.' || new_spring[0] == '?' {
        result += n_arr(&spring[1..], arr);
    }

    if new_spring[0] == '#' || new_spring[0] == '?' {
        if n <= spring.len()
            && spring[..n].contains('.') == false
            && (n == spring.len() || new_spring[n] != '#')
        {
            // if n == spring.len() {
            //     result += n_arr(&spring[n..], &arr[1..]);
            // } else {
            //     result += n_arr(&spring[n + 1..], &arr[1..]);
            // }
            result += n_arr(&spring[end..], &arr[1..]);
        }
    }

    result
}

fn part_1(lines: &[String]) -> usize {
    let mut damaged_springs: Vec<String> = Vec::new();
    let mut arrangments: Vec<Vec<usize>> = Vec::new();
    let mut sum: usize = 0;

    for line in lines {
        let line: Vec<&str> = line.split(' ').collect();
        let ds = line[0].to_string();
        let arr = line[1]
            .split(',')
            .map(|d| d.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        damaged_springs.push(ds);
        arrangments.push(arr);
    }

    for spring in 0..damaged_springs.len() {
        let n = n_arr(&damaged_springs[spring], &arrangments[spring]);
        println!("{}: {}", n, damaged_springs[spring]);
        sum += n;
    }
    // let a: usize = 1;
    //
    // let n = n_arr(&damaged_springs[a], &arrangments[a]);
    // println!("{}: {}", n, damaged_springs[a]);
    // sum = n;
    sum
}

fn main() -> Result<()> {
    testing(12, &part_1, 7771);
    Ok(())
}
