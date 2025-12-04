use std::error::Error;

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(4, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[InputEnt]) -> u64 {
    input
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter().enumerate().fold(0, |acc, (x, state)| {
                if *state == State::Paper && adjacent(input, x, y) < 4 {
                    acc + 1
                } else {
                    acc
                }
            })
        })
        .sum()
}

fn part2(input: &[InputEnt]) -> u64 {
    let mut board = input.to_vec();
    let w = board[0].len();
    let h = board.len();
    let mut removed = 0;

    loop {
        let mut this_removed = 0;

        for y in 0..h {
            for x in 0..w {
                if board[y][x] == State::Paper && adjacent(&board, x, y) < 4 {
                    board[y][x] = State::Empty;
                    this_removed += 1;
                }
            }
        }

        if this_removed == 0 {
            break;
        }

        removed += this_removed;
    }

    removed
}

fn adjacent(input: &[InputEnt], x: usize, y: usize) -> u64 {
    let mut result = 0;

    for (x, y) in adjacent_coords(input, x, y) {
        if input[y][x] == State::Paper {
            result += 1;
        }
    }

    result
}

fn adjacent_coords(input: &[InputEnt], x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::with_capacity(8);

    let w = input[0].len() as isize;
    let h = input.len() as isize;

    for ty in ((y as isize) - 1)..=((y as isize) + 1) {
        for tx in ((x as isize) - 1)..=((x as isize) + 1) {
            if ty >= 0 && ty < h && tx >= 0 && tx < w && !(tx == x as isize && ty == y as isize) {
                result.push((tx as usize, ty as usize));
            }
        }
    }

    result
}

// Input parsing

#[derive(Debug, Clone, PartialEq, Eq)]
enum State {
    Empty,
    Paper,
}

type InputEnt = Vec<State>;

fn input_transform(line: &str) -> InputEnt {
    line.chars()
        .map(|c| match c {
            '.' => State::Empty,
            '@' => State::Paper,
            c => panic!("Invalid char '{c}'"),
        })
        .collect()
}

#[cfg(test)]
mod tests;
