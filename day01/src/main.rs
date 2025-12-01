use std::error::Error;

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(1, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[InputEnt]) -> u64 {
    let mut cur_pos: i64 = 50;
    let mut password: u64 = 0;

    for movement in input {
        match movement {
            Rotation::Left(amount) => cur_pos = (cur_pos - *amount as i64).rem_euclid(100),
            Rotation::Right(amount) => cur_pos = (cur_pos + *amount as i64).rem_euclid(100),
        }

        if cur_pos == 0 {
            password += 1;
        }
    }

    password
}

fn part2(input: &[InputEnt]) -> u64 {
    let mut cur_pos: i64 = 50;
    let mut password: u64 = 0;

    for movement in input {
        let count = match movement {
            Rotation::Left(amount) => -(*amount as i64),
            Rotation::Right(amount) => *amount as i64,
        };

        let add = count.signum();

        for _ in 0..count.abs() {
            cur_pos += add;

            match cur_pos {
                -1 => cur_pos = 99,
                100 => cur_pos = 0,
                _ => (),
            }

            if cur_pos == 0 {
                password += 1;
            }
        }
    }

    password
}

// Input parsing

#[derive(Debug)]
enum Rotation {
    Left(u16),
    Right(u16),
}

type InputEnt = Rotation;

fn input_transform(line: &str) -> InputEnt {
    let (dir, amount) = line.split_at(1);

    let amount = amount.parse::<u16>().unwrap();

    match dir {
        "L" => Rotation::Left(amount),
        "R" => Rotation::Right(amount),
        _ => panic!("Invalid direction {dir}"),
    }
}

#[cfg(test)]
mod tests;
