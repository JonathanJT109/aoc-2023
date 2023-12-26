use anyhow::Result;
use aoc::print_answers;

fn prediction(sequence: &Vec<isize>, side: &char) -> isize {
    let mut pred_value: isize = 0;
    let mut seq: Vec<isize> = sequence.clone();
    let mut all_zeros: bool = false;
    let mut sign = -1;
    if *side == 'L' {
        pred_value = seq[0];
    }

    while all_zeros == false && seq.len() > 0 {
        let mut new_seq: Vec<isize> = Vec::new();
        let mut not_zero: bool = false;

        for i in 0..(seq.len() - 1) {
            let diff: isize = seq[i + 1] - seq[i];
            new_seq.push(diff);
            if diff != 0 {
                not_zero = true;
            }
        }

        if *side == 'R' {
            pred_value += seq[seq.len() - 1];
        } else {
            pred_value += new_seq[0] * sign;
            sign *= -1;
        }
        seq = new_seq;
        all_zeros = not_zero == false;
    }

    pred_value
}

fn part_1(lines: &[String]) -> isize {
    let mut answer: isize = 0;
    let history: Vec<Vec<isize>> = lines
        .iter()
        .map(|line| {
            line.split(' ')
                .map(|num| num.parse::<isize>().unwrap())
                .collect()
        })
        .collect();

    for seq in &history {
        answer += prediction(seq, &'R');
    }

    answer
}

fn part_2(lines: &[String]) -> isize {
    let mut answer: isize = 0;
    let history: Vec<Vec<isize>> = lines
        .iter()
        .map(|line| {
            line.split(' ')
                .map(|num| num.parse::<isize>().unwrap())
                .collect()
        })
        .collect();

    for seq in &history {
        answer += prediction(seq, &'L');
    }

    answer
}

fn main() -> Result<()> {
    print_answers(9, &part_1, &part_2);
    Ok(())
}
