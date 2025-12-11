use std::{collections::HashMap, error::Error};

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(11, input_transform)?;
    let map = create_map(&input);

    // Run parts
    println!("Part 1: {}", part1(&map));
    println!("Part 2: {}", part2(&map));

    Ok(())
}

fn part1(map: &HashMap<String, Vec<String>>) -> u64 {
    let mut seen = HashMap::new();

    walk1(map, "you", &mut seen)
}

fn walk1(map: &HashMap<String, Vec<String>>, from: &str, seen: &mut HashMap<String, u64>) -> u64 {
    // Look for onward connections
    if let Some(to) = map.get(from) {
        // Iterate onward connections summing number of routes
        to.iter()
            .map(|to| {
                // At the end?
                if to == "out" {
                    // Return single route
                    1
                } else {
                    // Seen this location before?
                    match seen.get(to) {
                        Some(routes) => {
                            // Yes - return number of routes found
                            *routes
                        }
                        None => {
                            // No - recurse route
                            let routes = walk1(map, to, seen);

                            // Insert in to seen routes
                            seen.insert(to.clone(), routes);

                            // Return number of routes
                            routes
                        }
                    }
                }
            })
            .sum()
    } else {
        // No route to anywhere
        0
    }
}

fn part2(map: &HashMap<String, Vec<String>>) -> u64 {
    let mut seen = HashMap::new();

    walk2(map, "svr", &mut seen, false, false)
}

fn walk2(
    map: &HashMap<String, Vec<String>>,
    from: &str,
    seen: &mut HashMap<(String, bool, bool), u64>,
    mut fft: bool,
    mut dac: bool,
) -> u64 {
    // Check for special nodes
    match from {
        "fft" => fft = true,
        "dac" => dac = true,
        _ => (),
    }

    // Look for onward connections
    if let Some(to) = map.get(from) {
        // Iterate onward connections summing number of routes
        to.iter()
            .map(|to| {
                // At the end?
                if to == "out" {
                    // Return single route if we've passed through both special nodes
                    if fft && dac { 1 } else { 0 }
                } else {
                    // Seen this location before and special node combination before?
                    match seen.get(&(to.to_string(), fft, dac)) {
                        Some(routes) => {
                            // Yes - return number of routes found
                            *routes
                        }
                        None => {
                            // No - recurse route
                            let routes = walk2(map, to, seen, fft, dac);

                            // Insert in to seen routes
                            seen.insert((to.clone(), fft, dac), routes);

                            // Return number of routes
                            routes
                        }
                    }
                }
            })
            .sum()
    } else {
        0
    }
}

// Input parsing

struct InputEnt {
    from: String,
    to: Vec<String>,
}

fn input_transform(line: &str) -> InputEnt {
    let mut iter = line.split_ascii_whitespace();

    let from = iter.next().unwrap().trim_end_matches(':').to_string();

    let to = iter.map(|s| s.to_string()).collect::<Vec<_>>();

    InputEnt { from, to }
}

fn create_map(input: &[InputEnt]) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();

    for conn in input {
        map.insert(conn.from.clone(), conn.to.clone());
    }

    map
}

#[cfg(test)]
mod tests;
