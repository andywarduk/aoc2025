#![feature(new_range_api)]

use std::{error::Error, range::RangeInclusive};

use aoc::input::parse_input_line;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_line(2, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &InputEnt) -> u64 {
    let mut result = 0;

    for range in input {
        for num in range.iter() {
            // Convert to string
            let numstr = format!("{}", num);

            // Even number of digits?
            if numstr.len() % 2 != 0 {
                continue;
            }

            // Get length to check
            let checklen = numstr.len() / 2;

            // Check the string
            if numstr[0..checklen] == numstr[checklen..] {
                result += num;
            }
        }
    }

    result
}

fn part2(input: &InputEnt) -> u64 {
    let mut result = 0;

    for range in input {
        for num in range.iter() {
            // Convert to string
            let numstr = format!("{}", num);

            // Convert to byte ptr
            let numbytes = numstr.as_bytes();

            // Loop valid check lengths
            for checklen in 1..=(numbytes.len() / 2) {
                if numbytes.len() % checklen != 0 {
                    continue;
                }

                // Chunk bytes by check length
                let mut chunks = numbytes.chunks_exact(checklen);

                // Get first chunk
                let first = chunks.next().unwrap();

                // Are all of the other chunks identical?
                if !chunks.any(|chunk| chunk != first) {
                    // Yes - got a result
                    result += num;

                    break;
                }
            }
        }
    }

    result
}

// Input parsing

type InputEnt = Vec<RangeInclusive<u64>>;

fn input_transform(line: &str) -> InputEnt {
    // Split by comma
    line.split(",")
        .map(|range| {
            if let Some((n1, n2)) = range.split_once('-') {
                RangeInclusive::from(n1.parse().unwrap()..=n2.parse().unwrap())
            } else {
                panic!("Invalid range {range}");
            }
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests;
