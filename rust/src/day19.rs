use anyhow::Result;
use aoc::testing;

#[derive(Debug)]
struct Workflow {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Workflow {
    fn new(x: usize, m: usize, a: usize, s: usize) -> Self {
        Self { x, m, a, s }
    }
}

fn parse(lines: &[String]) -> Vec<Workflow> {
    lines[1]
        .split('\n')
        .map(|line| {
            let line = line
                .trim_matches('{')
                .trim_matches('}')
                .split(',')
                .collect::<Vec<_>>();
            let mut workflow = Workflow::new(0, 0, 0, 0);
            for pair in line {
                let pair: Vec<&str> = pair.split('=').collect();
                if let [key, value] = pair.as_slice() {
                    match *key {
                        "x" => workflow.x = value.parse::<usize>().unwrap(),
                        "m" => workflow.m = value.parse::<usize>().unwrap(),
                        "a" => workflow.a = value.parse::<usize>().unwrap(),
                        _ => workflow.s = value.parse::<usize>().unwrap(),
                    }
                }
            }
            workflow
        })
        .collect()
}

fn part_1(lines: &[String]) -> usize {
    let lines = lines
        .join("\n")
        .split("\n\n")
        .map(|a| a.to_string())
        .collect::<Vec<_>>();

    for workflow in &workflows {
        println!("{:?}", workflow);
    }

    0
}

fn main() -> Result<()> {
    testing(19, &part_1, 19114);
    Ok(())
}
