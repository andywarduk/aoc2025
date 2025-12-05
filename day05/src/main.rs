use std::{error::Error, ops::RangeInclusive};

use aoc::input::parse_input;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let (ranges, ingredients) = parse_input(5, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&ranges, &ingredients));
    println!("Part 2: {}", part2(&ranges));

    Ok(())
}

fn part1(ranges: &[RangeInclusive<u64>], ingredients: &[u64]) -> u64 {
    // Iterate ingredients looking for any ranges which include it
    ingredients
        .iter()
        .filter(|&ingredient| ranges.iter().any(|range| range.contains(ingredient)))
        .count() as u64
}

fn part2(ranges: &[RangeInclusive<u64>]) -> u64 {
    // Normalise and count the total number of items in all of the ranges
    normalise_ranges(ranges)
        .map(|range| *range.end() - *range.start() + 1)
        .sum::<u64>()
}

fn normalise_ranges(ranges: &[RangeInclusive<u64>]) -> impl Iterator<Item = RangeInclusive<u64>> {
    let mut ranges_in = ranges.to_vec();
    let mut ranges_out: Vec<RangeInclusive<u64>> = Vec::with_capacity(ranges.len());

    loop {
        let mut some_combined = false;

        // Iterate the in ranges
        for in_range in &ranges_in {
            let mut combined = false;

            // Iterate the out ranges (starts empty)
            for out_range in &mut ranges_out {
                // Do the ranges combine?
                if let Some(combined_range) = combine_ranges(in_range, out_range) {
                    // Yes - set out range to the combined range
                    *out_range = combined_range;
                    combined = true;
                    break;
                }
            }

            // Combined with another?
            if combined {
                // Yes
                some_combined = true;
            } else {
                // No - add to out ranges
                ranges_out.push(in_range.clone());
            }
        }

        // Did we combine any?
        if !some_combined {
            // No - finished
            break;
        }

        // In ranges is old out ranges
        ranges_in = ranges_out;

        // Empty out ranges for next iteration
        ranges_out = Vec::with_capacity(ranges.len());
    }

    // Return out ranges iterator
    ranges_out.into_iter()
}

fn combine_ranges(
    r1: &RangeInclusive<u64>,
    r2: &RangeInclusive<u64>,
) -> Option<RangeInclusive<u64>> {
    // Work out if the ranges overlap
    if r1.start() < r2.start() && r1.end() >= r2.start() {
        // Overlaps at the start of out range
        Some((*r1.start())..=(*r1.end().max(r2.end())))
    } else if r1.start() <= r2.end() && r1.end() >= r2.end() {
        // Overlaps at the end of out range
        Some((*r1.start().min(r2.start()))..=(*r1.end()))
    } else if r1.start() >= r2.start() && r1.end() <= r2.end() {
        // In inside
        Some(r2.clone())
    } else if r2.start() >= r1.start() && r2.end() <= r1.end() {
        // Out inside
        Some(r1.clone())
    } else {
        None
    }
}

// Input parsing

type InputEnt = (Vec<RangeInclusive<u64>>, Vec<u64>);

fn input_transform(file: &str) -> InputEnt {
    let (ranges, ingredients) = file.split_once("\n\n").unwrap();

    let ranges = ranges
        .lines()
        .map(|line| {
            let (from, to) = line.split_once("-").unwrap();
            (from.parse::<u64>().unwrap())..=(to.parse::<u64>().unwrap())
        })
        .collect::<Vec<_>>();

    let ingredients = ingredients
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect();

    (ranges, ingredients)
}

#[cfg(test)]
mod tests;
