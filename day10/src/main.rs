use std::{
    collections::{HashSet, VecDeque},
    error::Error,
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

fn fewest_presses2(target: &[u16], additions: &[Vec<u8>]) -> u64 {
    let buttons = additions
        .iter()
        .map(|addition| {
            let mut out_vec = vec![0u8; target.len()];

            for &idx in addition {
                out_vec[idx as usize] += 1;
            }

            out_vec
        })
        .collect::<Vec<_>>();

    let cfg = z3::Config::new();

    z3::with_z3_config(&cfg, || {
        let optimize = z3::Optimize::new();

        // Create integer variables for each button count
        let button_vars: Vec<z3::ast::Int> = (0..buttons.len())
            .map(|i| z3::ast::Int::new_const(i as u32))
            .collect();

        // Add constraints: each button count must be non-negative
        for var in &button_vars {
            optimize.assert(&var.ge(z3::ast::Int::from_i64(0)));
        }

        // Add constraints: sum of (button_count * button_effect) must equal target
        for (dim, &target_val) in target.iter().enumerate() {
            let mut sum = z3::ast::Int::from_i64(0);

            for (button_idx, var) in button_vars.iter().enumerate() {
                let coeff = z3::ast::Int::from_i64(buttons[button_idx][dim] as i64);
                sum += var * &coeff;
            }

            optimize.assert(&sum.eq(z3::ast::Int::from_i64(target_val as i64)));
        }

        // Minimize the sum of button presses
        let mut total = z3::ast::Int::from_i64(0);

        for var in &button_vars {
            total += var;
        }

        optimize.minimize(&total);

        match optimize.check(&[]) {
            z3::SatResult::Sat => {
                if let Some(model) = optimize.get_model() {
                    match model.eval(&total, true) {
                        Some(val) => val.as_u64().unwrap_or(0),
                        None => 0,
                    }
                } else {
                    0
                }
            }
            _ => 0,
        }
    })
}

// Input parsing

#[derive(Debug)]
struct Machine {
    indicators: Vec<bool>,
    schematics: Vec<Vec<u8>>,
    joltages: Vec<u16>,
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
        .map(|nstr| nstr.parse::<u16>().unwrap())
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
