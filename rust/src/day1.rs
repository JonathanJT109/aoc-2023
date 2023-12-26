use anyhow::Result;
use aoc::print_answers;
/*
*
* 1 a b 8 c e
* -     -
* a b 1 c c e
* -         -
*   -       -
*     -     -
*     -   -
*     - -
*     -
*/

fn part_1(lines: &[String]) -> usize {
    let mut result = Vec::<usize>::new();

    for line in lines {
        let mut a: usize = 0;
        let mut b: usize = line.len() - 1;
        let text: Vec<char> = line.chars().collect();

        while a < b {
            if !text[a].is_ascii_digit() {
                a += 1;
            }

            if !text[b].is_ascii_digit() {
                b -= 1;
            }

            if text[a].is_ascii_digit() && text[b].is_ascii_digit() {
                let mut number = text[a] as usize - '0' as usize;
                number = number * 10 + text[b] as usize - '0' as usize;
                result.push(number);

                break;
            }
        }
    }

    result.iter().sum()
}

// Combini ng the first didgit and the last digit -> single two-digit number
// one two three four five six seven nine

fn part_2(lines: &[String]) -> usize {
    let mut result = Vec::<String>::new();

    for line in lines {
        let mut new_line: String = "".to_string();
        let text: Vec<char> = line.chars().collect();
        let mut i = 0;

        while i < text.len() {
            let letter = text[i];

            if letter.is_ascii_digit() {
                new_line.push(letter);
                i += 1;
                continue;
            }

            match letter {
                'o' => {
                    if i + 2 < text.len() && text[i + 1] == 'n' && text[i + 2] == 'e' {
                        new_line.push('1');
                        i += 3;
                        continue;
                    }
                }
                't' => {
                    if i + 2 < text.len() && text[i + 1] == 'w' && text[i + 2] == 'o' {
                        new_line.push('2');
                        i += 3;
                        continue;
                    }
                    if i + 4 <= text.len()
                        && text[i + 1] == 'h'
                        && text[i + 2] == 'r'
                        && text[i + 3] == 'e'
                        && text[i + 4] == 'e'
                    {
                        new_line.push('3');
                        i += 5;
                        continue;
                    }
                }
                'f' => {
                    if i + 3 < text.len() {
                        if text[i + 1] == 'o' && text[i + 2] == 'u' && text[i + 3] == 'r' {
                            new_line.push('4');
                            i += 4;
                            continue;
                        }
                        if text[i + 1] == 'i' && text[i + 2] == 'v' && text[i + 3] == 'e' {
                            new_line.push('5');
                            i += 4;
                            continue;
                        }
                    }
                }
                's' => {
                    if i + 2 < text.len() && text[i + 1] == 'i' && text[i + 2] == 'x' {
                        new_line.push('6');
                        i += 3;
                        continue;
                    }
                    if i + 4 < text.len()
                        && text[i + 1] == 'e'
                        && text[i + 2] == 'v'
                        && text[i + 3] == 'e'
                        && text[i + 4] == 'n'
                    {
                        new_line.push('7');
                        i += 5;
                        continue;
                    }
                }
                'e' => {
                    if i + 4 < text.len()
                        && text[i + 1] == 'i'
                        && text[i + 2] == 'g'
                        && text[i + 3] == 'h'
                        && text[i + 4] == 't'
                    {
                        new_line.push('8');
                        i += 5;
                        continue;
                    }
                }
                'n' => {
                    if i + 3 < text.len()
                        && text[i + 1] == 'i'
                        && text[i + 2] == 'n'
                        && text[i + 3] == 'e'
                    {
                        new_line.push('9');
                        i += 4;
                        continue;
                    }
                }
                _ => (),
            }
            new_line.push(letter);
            i += 1;
        }
        result.push(new_line);
    }

    part_1(&result)
}

fn main() -> Result<()> {
    print_answers(1, &part_1, &part_2);
    Ok(())
}
