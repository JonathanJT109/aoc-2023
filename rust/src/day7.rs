use anyhow::Result;
use aoc::read_file;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug)]
struct Hand {
    hand: Vec<char>,
    bid: usize,
}

fn kind(hand: &Hand) -> usize {
    let mut identifiers: HashMap<char, usize> = HashMap::new();

    for card in &hand.hand {
        let count = identifiers.entry(*card).or_insert(0);
        *count += 1;
    }

    match identifiers.len() {
        1 => 6,
        2 => {
            let a = identifiers.get(&hand.hand[0]).unwrap();
            if *a == 4 || *a == 1 {
                return 5;
            }
            4
        }
        3 => {
            for l in &hand.hand {
                if *identifiers.get(l).unwrap() == 3 as usize {
                    return 3;
                }
            }
            2
        }
        4 => 1,
        _ => 0,
    }
}

fn kind2(hand: &Hand) -> usize {
    let mut identifiers: HashMap<char, usize> = HashMap::new();
    let mut j = 0;
    let mut max_n = 0;
    let mut card_: char = 'J';

    for card in &hand.hand {
        if *card == 'J' {
            j += 1;
            continue;
        }
        let count = identifiers.entry(*card).or_insert(0);
        *count += 1;
        if *count > max_n {
            max_n = *count;
            card_ = *card;
        }
    }

    if j > 0 {
        let count = identifiers.entry(card_).or_default();
        *count += j;
    }

    match identifiers.len() {
        1 => 6,
        2 => {
            let a = identifiers.iter().next().unwrap();
            if *a.1 == 4 || *a.1 == 1 {
                return 5;
            }
            4
        }
        3 => {
            for l in &hand.hand {
                if *l != 'J' && *identifiers.get(l).unwrap() == 3 as usize {
                    return 3;
                }
            }
            2
        }
        4 => 1,
        _ => 0,
    }
}

fn compare(
    hand1: &Hand,
    hand2: &Hand,
    value: &HashMap<char, usize>,
    cmp_fn: &dyn Fn(&Hand) -> usize,
) -> Ordering {
    let kind_1 = cmp_fn(hand1);
    let kind_2 = cmp_fn(hand2);

    if kind_1 == kind_2 {
        for i in 0..hand1.hand.len() {
            let value1 = *value.get(&hand1.hand[i]).unwrap();
            let value2 = *value.get(&hand2.hand[i]).unwrap();
            if value1 > value2 {
                return Ordering::Greater;
            } else if value1 < value2 {
                return Ordering::Less;
            }
        }
    } else if kind_1 > kind_2 {
        return Ordering::Greater;
    }
    return Ordering::Less;
}

fn part_1(lines: &[String]) -> usize {
    let mut hands: Vec<Hand> = lines
        .iter()
        .map(|line| {
            let mut a = line.split(' ');
            let hand = a.clone().nth(0).unwrap().chars().collect::<Vec<char>>();
            let bid = a.nth(1).unwrap().parse::<usize>().unwrap();
            Hand { hand, bid }
        })
        .collect();

    let card_value: HashMap<char, usize> = HashMap::from([
        ('A', 13),
        ('K', 12),
        ('Q', 11),
        ('J', 10),
        ('T', 9),
        ('9', 8),
        ('8', 7),
        ('7', 6),
        ('6', 5),
        ('5', 4),
        ('4', 3),
        ('3', 2),
        ('2', 1),
    ]);

    hands.sort_by(|a, b| compare(a, b, &card_value, &kind));

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum::<usize>()
}

fn part_2(lines: &[String]) -> usize {
    let mut hands: Vec<Hand> = lines
        .iter()
        .map(|line| {
            let mut a = line.split(' ');
            let hand = a.clone().nth(0).unwrap().chars().collect::<Vec<char>>();
            let bid = a.nth(1).unwrap().parse::<usize>().unwrap();
            Hand { hand, bid }
        })
        .collect();

    let card_value: HashMap<char, usize> = HashMap::from([
        ('A', 13),
        ('K', 12),
        ('Q', 11),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
        ('J', 1),
    ]);

    hands.sort_by(|a, b| compare(a, b, &card_value, &kind2));

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum::<usize>()
}

fn main() -> Result<()> {
    if let Ok(file) = read_file("./src/input/day7.txt") {
        let answer = part_1(&file);
        println!("Answer: {answer}");
    } else {
        eprintln!("ERROR: File not found")
    }

    if let Ok(file) = read_file("./src/input/day7.txt") {
        let answer = part_2(&file);
        println!("Answer: {answer}");
    } else {
        eprintln!("ERROR: File not found")
    }
    Ok(())
}
