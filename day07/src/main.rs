use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(7, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[InputEnt]) -> u64 {
    // Find start
    let mut beams = HashSet::new();

    let (sx, mut y) = find_start(input);

    beams.insert(sx);

    let mut splits = 0;
    y += 1;

    while y < input.len() {
        let mut next_beams = HashSet::new();

        for x in beams {
            match input[y][x] {
                Space::Splitter => {
                    next_beams.insert(x - 1);
                    next_beams.insert(x + 1);
                    splits += 1;
                }
                Space::Empty => {
                    next_beams.insert(x);
                }
                _ => panic!(),
            }
        }

        y += 1;
        beams = next_beams;
    }

    splits
}

fn part2(input: &[InputEnt]) -> u64 {
    // Find start
    let mut beams = HashMap::new();

    let (sx, mut y) = find_start(input);

    beams.insert(sx, 1);

    y += 1;

    while y < input.len() {
        let mut next_beams = HashMap::new();

        for (x, count) in beams {
            match input[y][x] {
                Space::Splitter => {
                    *next_beams.entry(x - 1).or_insert(0) += count;
                    *next_beams.entry(x + 1).or_insert(0) += count;
                }
                Space::Empty => {
                    *next_beams.entry(x).or_insert(0) += count;
                }
                _ => panic!(),
            }
        }

        y += 1;
        beams = next_beams;
    }

    beams.values().sum::<usize>() as u64
}

fn find_start(input: &[InputEnt]) -> (usize, usize) {
    input
        .iter()
        .enumerate()
        .filter_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(x, s)| {
                    if *s == Space::Start {
                        Some((x, y))
                    } else {
                        None
                    }
                })
                .next()
        })
        .next()
        .unwrap()
}

// Input parsing

#[derive(Debug, PartialEq, Eq)]
enum Space {
    Start,
    Empty,
    Splitter,
}

type InputEnt = Vec<Space>;

fn input_transform(line: &str) -> InputEnt {
    line.chars()
        .map(|c| match c {
            '.' => Space::Empty,
            '^' => Space::Splitter,
            'S' => Space::Start,
            _ => panic!(),
        })
        .collect()
}

#[cfg(test)]
mod tests;
