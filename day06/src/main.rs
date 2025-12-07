use std::error::Error;

use aoc::input::parse_input;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input 1
    let (numbers, operators) = parse_input(6, input_transform1)?;

    // Run part 1
    println!("Part 1: {}", part1(&numbers, &operators));

    // Get input 1
    let (numbers, operators) = parse_input(6, input_transform2)?;

    // Run part 2
    println!("Part 2: {}", part2(&numbers, &operators));

    Ok(())
}

fn part1(numbers: &[Vec<u64>], operators: &[char]) -> u64 {
    // Iterate operators and apply to each column. Sum the results
    operators
        .iter()
        .enumerate()
        .map(|(idx, op)| match op {
            '+' => numbers.iter().fold(0, |acc, numarr| numarr[idx] + acc),
            '*' => numbers.iter().fold(1, |acc, numarr| numarr[idx] * acc),
            _ => panic!(),
        })
        .sum()
}

fn part2(numbers: &[Vec<String>], operators: &[char]) -> u64 {
    // Iterate operators
    operators
        .iter()
        .enumerate()
        .map(|(col, op)| {
            // Get length of this column
            let numlen = numbers[0][col].len();

            // Get initial fold value
            let init = match op {
                '+' => 0,
                '*' => 1,
                _ => panic!(),
            };

            // Fold each number in to the result
            (0..numlen).fold(init, |acc, pos| {
                let num = numbers
                    .iter()
                    .map(|line| &line[col][pos..(pos + 1)])
                    .collect::<String>()
                    .trim()
                    .parse::<u64>()
                    .unwrap();

                // Apply the operator
                match op {
                    '+' => acc + num,
                    '*' => acc * num,
                    _ => panic!(),
                }
            })
        })
        .sum()
}

// Input parsing

fn input_transform1(file: &str) -> (Vec<Vec<u64>>, Vec<char>) {
    let mut numbers = Vec::new();
    let mut operators = Vec::new();

    for line in file.lines() {
        match line.split_ascii_whitespace().next() {
            Some("*") | Some("+") => {
                // Operators line
                operators = line
                    .split_ascii_whitespace()
                    .map(|op| match op {
                        "+" => '+',
                        "*" => '*',
                        op => panic!("Invald operator {op}"),
                    })
                    .collect::<Vec<_>>();
            }
            _ => {
                // Numbers line
                numbers.push(
                    line.split_ascii_whitespace()
                        .map(|numstr| numstr.parse::<u64>().unwrap())
                        .collect::<Vec<_>>(),
                );
            }
        }
    }

    (numbers, operators)
}

fn input_transform2(file: &str) -> (Vec<Vec<String>>, Vec<char>) {
    let mut operators = String::new();

    // Get operators line
    for line in file.lines() {
        match line.split_ascii_whitespace().next() {
            Some("*") | Some("+") => {
                operators = line.to_string();
                break;
            }
            _ => (),
        }
    }

    // Get positions of the operators
    let positions = operators
        .chars()
        .enumerate()
        .filter_map(|(idx, c)| {
            if matches!(c, '+' | '*') {
                Some(idx)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    // Build return operators vec
    let operators = positions
        .iter()
        .map(|p| operators[*p..].chars().next().unwrap())
        .collect();

    // Build numbers string table
    let numbers = file
        .lines()
        .filter_map(|line| match line.split_ascii_whitespace().next() {
            Some("*") | Some("+") => None,
            _ => {
                let nums = positions
                    .iter()
                    .enumerate()
                    .map(|(idx, p)| {
                        let s = if idx + 1 == positions.len() {
                            &line[*p..]
                        } else {
                            &line[*p..(positions[idx + 1] - 1)]
                        };

                        s.to_string()
                    })
                    .collect::<Vec<_>>();

                Some(nums)
            }
        })
        .collect::<Vec<_>>();

    (numbers, operators)
}

#[cfg(test)]
mod tests;
