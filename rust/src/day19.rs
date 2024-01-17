use anyhow::Result;
use aoc::print_answers;
use std::collections::HashMap;
use std::ops::Index;

#[derive(Debug)]
struct Workflow {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
    valid: bool,
}

#[derive(Debug, PartialEq, Eq)]
enum Comparison {
    Less,
    Greater,
}

impl Comparison {
    fn compare<T: PartialOrd>(&self, a: T, b: T) -> bool {
        match self {
            Comparison::Greater => a > b,
            Comparison::Less => a < b,
        }
    }
}

#[derive(Debug)]
struct Rule {
    letter: char,
    cmp: Comparison,
    num: usize,
    dest: String,
}

impl Rule {
    fn new(letter: char, comp: Comparison, num: usize, dest: String) -> Self {
        Self {
            letter,
            cmp: comp,
            num,
            dest,
        }
    }

    fn parse(rule: &[char]) -> Rule {
        let comp = match rule[1] {
            '>' => Comparison::Greater,
            _ => Comparison::Less,
        };

        let i = rule.iter().position(|&c| c == ':').unwrap();
        let num = rule[2..i]
            .iter()
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        let dest = rule[i + 1..].iter().collect::<String>();
        Self::new(rule[0], comp, num, dest)
    }
}

#[derive(Debug)]
struct Rating {
    rules: Vec<Rule>,
    last: String,
}

type Ratings = HashMap<String, Rating>;

impl Workflow {
    fn new(x: usize, m: usize, a: usize, s: usize) -> Self {
        Self {
            x,
            m,
            a,
            s,
            valid: false,
        }
    }

    fn parse(lines: &[String]) -> Vec<Workflow> {
        lines[1]
            .split('\n')
            .map(|line| {
                let line = line
                    .trim_matches(|c| c == '{' || c == '}')
                    .split(',')
                    .collect::<Vec<_>>();
                let mut workflow = Self::new(0, 0, 0, 0);
                for pair in line {
                    let pair: Vec<&str> = pair.split('=').collect();
                    if let [key, value] = pair.as_slice() {
                        let value = value.parse::<usize>().unwrap();
                        match *key {
                            "x" => workflow.x = value,
                            "m" => workflow.m = value,
                            "a" => workflow.a = value,
                            _ => workflow.s = value,
                        }
                    }
                }
                workflow
            })
            .collect()
    }
}

impl Index<char> for Workflow {
    type Output = usize;

    fn index(&self, index: char) -> &usize {
        match index {
            'x' => &self.x,
            'm' => &self.m,
            'a' => &self.a,
            's' => &self.s,
            _ => panic!("Invalid index"),
        }
    }
}

fn parse_ratings(lines: &[String]) -> Ratings {
    let lines = lines[0]
        .split('\n')
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut final_hash: Ratings = HashMap::new();

    for line in &lines {
        let i = line.iter().position(|&c| c == '{').unwrap();
        let key = line[..i].iter().collect::<String>();
        let new_line = line[(i + 1)..(line.len() - 1)].iter().collect::<String>();
        let mut rules: Vec<Rule> = Vec::new();
        let mut final_dest: String = "".to_string();

        for rating in new_line
            .split(',')
            .map(|line| line.chars().collect::<Vec<_>>())
        {
            if rating.contains(&'>') || rating.contains(&'<') {
                let rule = Rule::parse(&rating);
                rules.push(rule);
            }
            final_dest = rating.iter().collect::<String>();
        }
        final_hash.insert(
            key,
            Rating {
                rules,
                last: final_dest,
            },
        );
    }

    final_hash
}

fn part_1(lines: &[String]) -> usize {
    let lines = lines
        .join("\n")
        .split("\n\n")
        .map(|a| a.to_string())
        .collect::<Vec<_>>();

    let mut workflows = Workflow::parse(&lines);
    let hash = parse_ratings(&lines);

    for workflow in &mut workflows {
        let mut curr: &Rating;
        let mut ch = "in".to_string();

        while ch != "A" && ch != "R" {
            curr = hash.get(&ch).unwrap();
            ch = curr.last.clone();
            for rule in curr.rules.iter() {
                if rule.cmp.compare(workflow[rule.letter], rule.num) {
                    ch = rule.dest.clone();
                    break;
                }
            }
        }

        if ch == "A" {
            workflow.valid = true;
        }
    }

    workflows
        .iter()
        .filter(|x| x.valid)
        .map(|x| x.x + x.m + x.a + x.s)
        .sum()
}

fn count(ranges: &mut HashMap<char, (usize, usize)>, name: String, hash: &Ratings) -> usize {
    if name == "R" {
        return 0;
    }

    if name == "A" {
        let mut product = 1;
        for (low, high) in ranges.values() {
            product *= *high - *low + 1;
        }
        return product;
    }

    let curr = hash.get(&name).unwrap();

    let mut total = 0;

    for rule in &curr.rules {
        let (lo, hi) = ranges.get(&rule.letter).unwrap();
        let t;
        let f;

        if rule.cmp == Comparison::Less {
            t = (*lo, *hi.min(&(rule.num - 1)));
            f = (*lo.max(&rule.num), *hi);
        } else {
            t = (*lo.max(&(rule.num + 1)), *hi);
            f = (*lo, *hi.min(&rule.num));
        }

        if t.0 <= t.1 {
            let mut copy = ranges.clone();
            copy.insert(rule.letter, t);
            total += count(&mut copy, rule.dest.clone(), hash);
        }

        if f.0 <= f.1 {
            *ranges = ranges.clone();
            ranges.insert(rule.letter, f);
        } else {
            break;
        }
    }
    total += count(ranges, curr.last.clone(), hash);

    total
}

fn part_2(lines: &[String]) -> usize {
    let lines = lines
        .join("\n")
        .split("\n\n")
        .map(|a| a.to_string())
        .collect::<Vec<_>>();

    let hash = parse_ratings(&lines);
    let mut ranges: HashMap<char, (usize, usize)> = HashMap::from([
        ('x', (1, 4000)),
        ('m', (1, 4000)),
        ('a', (1, 4000)),
        ('s', (1, 4000)),
    ]);

    count(&mut ranges, "in".to_string(), &hash)
}

fn main() -> Result<()> {
    print_answers(19, &part_1, &part_2);
    Ok(())
}
