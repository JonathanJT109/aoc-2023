use anyhow::Result;
use aoc::testing;
use std::collections::HashMap;

// TODO: Understand the algorithm
// TODO: Break it down into part_1 and part_2

fn n_arr<'a>(spring: &'a str, arr: &'a [usize]) -> usize {
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
            && !spring[..n].contains('.')
            && (n == spring.len() || new_spring[n] != '#')
        {
            result += n_arr(&spring[end..], &arr[1..]);
        }
    }

    result
}

fn n_arr2<'a>(
    cache: &mut HashMap<(&'a str, &'a [usize]), usize>,
    spring: &'a str,
    arr: &'a [usize],
) -> usize {
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

    let key = (spring.clone(), arr.clone());

    if let Some(&result) = cache.get(&key) {
        return result;
    }

    let mut result: usize = 0;
    let n = arr[0];
    let end: usize = (n + 1).min(spring.len());

    if new_spring[0] == '.' || new_spring[0] == '?' {
        result += n_arr2(cache, &spring[1..], arr);
    }

    if new_spring[0] == '#' || new_spring[0] == '?' {
        if n <= spring.len()
            && !spring[..n].contains('.')
            && (n == spring.len() || new_spring[n] != '#')
        {
            result += n_arr2(cache, &spring[end..], &arr[1..]);
        }
    }

    cache.insert(key, result);

    result
}

fn part_1(lines: &[String]) -> usize {
    let mut damaged_springs: Vec<String> = Vec::new();
    let mut arrangments: Vec<Vec<usize>> = Vec::new();
    let mut sum: usize = 0;

    for line in lines {
        let line: Vec<&str> = line.split(' ').collect();
        let ds = std::iter::repeat(line[0])
            .take(5)
            .collect::<Vec<&str>>()
            .join("?");

        let arr = line[1]
            .split(',')
            .map(|d| d.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
            .repeat(5);

        // let ds = line[0].to_string();
        // let arr = line[1]
        //     .split(',')
        //     .map(|d| d.parse::<usize>().unwrap())
        //     .collect::<Vec<usize>>();
        //
        damaged_springs.push(ds);
        arrangments.push(arr);
    }

    let mut cache = HashMap::new();

    for spring in 0..damaged_springs.len() {
        let n = n_arr2(&mut cache, &damaged_springs[spring], &arrangments[spring]);
        sum += n;
    }
    sum
}

fn main() -> Result<()> {
    testing(12, &part_1, 525152);
    Ok(())
}
