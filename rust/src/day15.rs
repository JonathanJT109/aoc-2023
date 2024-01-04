use anyhow::Result;
use aoc::print_answers;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Clone, Eq)]
struct Lens {
    label: String,
    focal_length: usize,
}

impl PartialEq for Lens {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

fn hash_algo(c: char, current: usize) -> usize {
    (current + c as usize) * 17 % 256
}

fn part_1(lines: &[String]) -> usize {
    let lines = lines[0].chars().collect::<Vec<_>>();
    let mut sum: usize = 0;
    let mut current: usize = 0;

    for (x, c) in lines.iter().enumerate() {
        if *c == ',' {
            sum += current;
            current = 0;
            continue;
        }

        current = hash_algo(*c, current);

        if x == lines.len() - 1 {
            sum += current;
        }
    }

    sum
}

fn part_2(lines: &[String]) -> usize {
    let lines = lines[0].split(',').collect::<Vec<_>>();
    let mut sum: usize = 0;
    let mut boxes: HashMap<usize, Vec<Lens>> = HashMap::new();

    for line in &lines {
        let line: Vec<char> = line.chars().collect();
        let mut index: usize = 0;
        let mut i: usize = 0;

        while i < line.len() && line[i] != '=' && line[i] != '-' {
            index = hash_algo(line[i], index);
            i += 1;
        }

        let lens: Lens = Lens {
            label: line[0..i].iter().collect(),
            focal_length: line[line.len() - 1].to_digit(10).unwrap_or(0) as usize,
        };

        match line[i] {
            '=' => {
                boxes
                    .entry(index)
                    .and_modify(|a| {
                        if let Some(b) = a.iter_mut().find(|b| *b == &lens) {
                            b.focal_length = lens.focal_length;
                        } else {
                            a.push(lens.clone());
                        }
                    })
                    .or_insert(vec![lens.clone()]);
            }
            _ => {
                if let Some(a) = boxes.get_mut(&index) {
                    if let Some(i) = a.iter().position(|b| *b == lens) {
                        a.remove(i);
                    }
                }
            }
        }
    }

    let library = boxes.iter().sorted_by(|a, b| Ord::cmp(a.0, b.0));

    for i in library {
        for (x, lens) in i.1.iter().enumerate() {
            sum += (*i.0 + 1) * (x + 1) * lens.focal_length;
        }
    }

    sum
}

fn main() -> Result<()> {
    print_answers(15, &part_1, &part_2);
    Ok(())
}
