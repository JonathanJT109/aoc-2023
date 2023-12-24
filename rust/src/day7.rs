use anyhow::Result;
use aoc::read_file;
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

fn compare(hand1: &Hand, hand2: &Hand, value: &HashMap<char, usize>) -> Option<usize> {
    let kind_1 = kind(hand1);
    let kind_2 = kind(hand2);

    if kind_1 == kind_2 {
        for i in 0..hand1.hand.len() {
            let value1 = *value.get(&hand1.hand[i]).unwrap();
            let value2 = *value.get(&hand2.hand[i]).unwrap();
            if value1 > value2 {
                return Some(1);
            } else if value1 < value2 {
                return Some(0);
            }
        }
    } else if kind_1 > kind_2 {
        return Some(1);
    }
    Some(0)
}

fn ranking(hands: &mut [Hand]) {
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

    for i in 0..hands.len() {
        for j in 0..(hands.len() - 1 - i) {
            match compare(&hands[j], &hands[j + 1], &card_value) {
                Some(1) => {
                    hands.swap(j, j + 1);
                }
                _ => (),
            };
        }
    }
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

    ranking(&mut hands);

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum::<usize>()
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

fn compare2(hand1: &Hand, hand2: &Hand, value: &HashMap<char, usize>) -> Option<usize> {
    let kind_1 = kind2(hand1);
    let kind_2 = kind2(hand2);

    if kind_1 == kind_2 {
        for i in 0..hand1.hand.len() {
            let value1 = *value.get(&hand1.hand[i]).unwrap();
            let value2 = *value.get(&hand2.hand[i]).unwrap();
            if value1 > value2 {
                return Some(1);
            } else if value1 < value2 {
                return Some(0);
            }
        }
    } else if kind_1 > kind_2 {
        return Some(1);
    }
    Some(0)
}

fn ranking2(hands: &mut [Hand]) {
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

    for i in 0..hands.len() {
        for j in 0..(hands.len() - 1 - i) {
            match compare2(&hands[j], &hands[j + 1], &card_value) {
                Some(1) => {
                    hands.swap(j, j + 1);
                }
                _ => (),
            };
        }
    }
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

    ranking2(&mut hands);

    for hand in &hands {
        println!("{:?}", hand);
    }

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum::<usize>()
}

fn main() -> Result<()> {
    // if let Ok(file) = read_file("./src/input/day7.txt") {
    //     let answer = part_1(&file);
    //     // 6440
    //     println!("{answer}");
    // }

    if let Ok(file) = read_file("./src/input/day7.txt") {
        let answer = part_2(&file);
        println!("{answer}");
    }
    Ok(())
}
