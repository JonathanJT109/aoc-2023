use anyhow::Result;
use aoc::print_answers;

fn add_index(games: &[String]) -> usize {
    let mut sum: usize = 0;

    for game in games {
        let words: Vec<&str> = game.split(' ').collect();
        let index = words[1];
        let index: usize = index[..index.len() - 1].parse().unwrap();
        sum += index;
    }

    sum
}

fn part_1(games: &[String]) -> usize {
    const RED_MAX: usize = 12;
    const GREEN_MAX: usize = 13;
    const BLUE_MAX: usize = 14;

    let mut valid_games = Vec::<String>::new();

    for game in games {
        let words: Vec<&str> = game.split(' ').collect();
        let mut check: bool = false;
        let mut a: usize = 2;

        while a < words.len() {
            let number: usize = words[a].parse().unwrap();
            a += 1;
            let word = words[a];

            if word.contains("blue") {
                check = number <= BLUE_MAX;
            } else if word.contains("red") {
                check = number <= RED_MAX;
            } else if word.contains("green") {
                check = number <= GREEN_MAX;
            }

            if !check {
                break;
            }

            a += 1;
        }

        if check {
            valid_games.push(game.clone());
        }
    }

    add_index(&valid_games)
}

fn part_2(games: &[String]) -> usize {
    let mut sum = 0;

    for game in games {
        let words: Vec<&str> = game.split(' ').collect();
        let mut a = 2;
        let mut min_blues = 0;
        let mut min_reds = 0;
        let mut min_greens = 0;

        while a < words.len() {
            let number = words[a].parse::<usize>().unwrap();
            a += 1;
            let word = words[a];

            if word.contains("blue") {
                min_blues = min_blues.max(number);
            } else if word.contains("red") {
                min_reds = min_reds.max(number);
            } else if word.contains("green") {
                min_greens = min_greens.max(number);
            }
            a += 1;
        }
        sum += min_blues * min_reds * min_greens;
    }

    sum
}

fn main() -> Result<()> {
    print_answers(2, &part_1, &part_2);
    Ok(())
}
