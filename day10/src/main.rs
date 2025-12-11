use std::{
    collections::{HashSet, VecDeque},
    error::Error,
    ops::Neg,
};

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(10, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[Machine]) -> u64 {
    input
        .iter()
        .map(|m| {
            let target = m
                .indicators
                .iter()
                .enumerate()
                .fold(0, |acc, (i, ind)| if *ind { acc | (1 << i) } else { acc });

            let bitmasks = m
                .schematics
                .iter()
                .map(|s| s.iter().fold(0, |acc, bit| acc | 1 << *bit))
                .collect::<Vec<_>>();

            fewest_presses1(target, &bitmasks)
        })
        .sum()
}

fn fewest_presses1(target: u64, bitmasks: &[u64]) -> u64 {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();

    queue.push_back((0, 0));
    seen.insert(0);

    while let Some((mut iters, state)) = queue.pop_front() {
        iters += 1;

        for b in bitmasks {
            let new_state = state ^ *b;

            if new_state == target {
                return iters;
            }

            if !seen.contains(&new_state) {
                seen.insert(new_state);
                queue.push_back((iters, new_state));
            }
        }
    }

    panic!()
}

fn part2(input: &[Machine]) -> u64 {
    input
        .iter()
        .map(|m| fewest_presses2(&m.joltages, &m.schematics))
        .sum()
}

fn fewest_presses2(target: &[u8], additions: &[Vec<u8>]) -> u64 {
    let mut min_amount = vec![u64::MAX; 256];

    // TODO

    0
}

// Input parsing

#[derive(Debug)]
struct Machine {
    indicators: Vec<bool>,
    schematics: Vec<Vec<u8>>,
    joltages: Vec<u8>,
}

fn input_transform(line: &str) -> Machine {
    let mut iter = line.split_ascii_whitespace();

    let indicators: Vec<bool> = iter
        .next()
        .unwrap()
        .chars()
        .filter_map(|c| match c {
            '.' => Some(false),
            '#' => Some(true),
            _ => None,
        })
        .collect();

    let joltages = iter
        .next_back()
        .unwrap()
        .trim_matches(|c| c == '{' || c == '}')
        .split(',')
        .map(|nstr| nstr.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    let schematics = iter
        .map(|s| {
            s.trim_matches(|c| c == '(' || c == ')')
                .split(',')
                .map(|nstr| nstr.parse::<u8>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Machine {
        indicators,
        schematics,
        joltages,
    }
}

#[cfg(test)]
mod tests;
