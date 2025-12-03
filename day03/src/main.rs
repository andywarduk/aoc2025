use std::error::Error;

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(3, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[InputEnt]) -> u64 {
    // Iterate banks
    input
        .iter()
        .map(|bank| {
            // Get highest digit (not last)
            let (p1, d1) =
                bank.iter()
                    .rev()
                    .skip(1)
                    .rev()
                    .enumerate()
                    .fold(
                        (0, 0),
                        |(mi, mv), (i, v)| {
                            if *v > mv { (i, *v) } else { (mi, mv) }
                        },
                    );

            // Get highest digit in the reaminder
            let d2 = bank.iter().skip(p1 + 1).max().unwrap();

            // Calculate joltage
            ((d1 * 10) + d2) as u64
        })
        .sum()
}

fn part2(input: &[InputEnt]) -> u64 {
    input
        .iter()
        .map(|bank| {
            let mut pos = 0;

            // Get 12 digits
            let digits = (0..12)
                .map(|digit| {
                    // Get highest digit from remaining digits, leaving enough for later iterations
                    let (new_pos, d) = bank
                        .iter()
                        .enumerate()
                        .skip(pos)
                        .rev()
                        .skip(11 - digit)
                        .rev()
                        .fold(
                            (0, 0),
                            |(mi, mv), (i, v)| {
                                if *v > mv { (i, *v) } else { (mi, mv) }
                            },
                        );

                    // Move to next start position
                    pos = new_pos + 1;

                    d
                })
                .collect::<Vec<_>>();

            // Build number
            digits
                .into_iter()
                .rev()
                .enumerate()
                .map(|(i, digit)| digit as u64 * 10u64.pow(i as u32))
                .sum::<u64>()
        })
        .sum()
}

// Input parsing

type InputEnt = Vec<u8>;

fn input_transform(line: &str) -> InputEnt {
    line.chars().map(|c| c as u8 - b'0').collect()
}

#[cfg(test)]
mod tests;
