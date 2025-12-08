use std::{collections::HashSet, error::Error};

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let junctions = parse_input_vec(8, input_transform)?;

    // Build edges
    let edges = build_edges(&junctions);

    // Run parts
    println!("Part 1: {}", part1(&junctions, &edges, 1000));
    println!("Part 2: {}", part2(&junctions, &edges));

    Ok(())
}

fn part1(junctions: &[Junction], edges: &[(usize, usize, f64)], join_count: usize) -> u64 {
    // Initialise circuits
    let mut circuits: Vec<HashSet<usize>> = (0..junctions.len())
        .map(|j| HashSet::from([j]))
        .collect::<Vec<_>>();

    // Take the shortest n edges
    for (j1, j2, _) in edges.iter().take(join_count) {
        // Get the positions of all of the circuits containing one of the nodes
        let mut positions = circuits
            .iter()
            .enumerate()
            .filter_map(|(i, circuit)| {
                if circuit.contains(j1) || circuit.contains(j2) {
                    Some(i)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .into_iter();

        // Get first circuit position
        let to = positions.next().unwrap();

        // Merge all of the other circuits in
        for from in positions {
            let from = circuits.swap_remove(from);
            circuits[to].extend(from);
        }
    }

    // Sort circuits by size descending
    circuits.sort_by_key(|b| std::cmp::Reverse(b.len()));

    // Get biggest 3 circuits and multiply their sizes together
    circuits
        .iter()
        .take(3)
        .map(|set| set.len() as u64)
        .product()
}

fn part2(junctions: &[Junction], edges: &[(usize, usize, f64)]) -> u64 {
    // Initialise circuits
    let mut circuits: Vec<HashSet<usize>> = (0..junctions.len())
        .map(|j| HashSet::from([j]))
        .collect::<Vec<_>>();

    // Initialise result
    let mut result = 0;

    // Iterate the edges in ascending length order
    for (j1, j2, _) in edges {
        // Get the positions of all of the circuits containing one of the nodes
        let mut positions = circuits
            .iter()
            .enumerate()
            .filter_map(|(i, circuit)| {
                if circuit.contains(j1) || circuit.contains(j2) {
                    Some(i)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .into_iter();

        // Get first circuit position
        let to = positions.next().unwrap();

        // Merge all of the other circuits in
        for from in positions {
            let from = circuits.swap_remove(from);
            circuits[to].extend(from);
        }

        // Only one circuit?
        if circuits.len() == 1 {
            // Yes - multiply the x coordinate of the two junctions
            result = junctions[*j1].coords[0] * junctions[*j2].coords[0];

            break;
        }
    }

    result
}

fn build_edges(junctions: &[Junction]) -> Vec<(usize, usize, f64)> {
    // Iterate junction combinations and calculate distance between for each
    let mut edges = junctions
        .iter()
        .enumerate()
        .flat_map(|(i1, j1)| {
            junctions
                .iter()
                .enumerate()
                .skip(i1 + 1)
                .map(move |(i2, j2)| (i1, i2, j1.distance_to(j2)))
        })
        .collect::<Vec<_>>();

    // Sort by length ascending
    edges.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    edges
}

// Input parsing

struct Junction {
    coords: Vec<u64>,
}

impl Junction {
    fn distance_to(&self, other: &Junction) -> f64 {
        let x = self.coords[0].abs_diff(other.coords[0]);
        let y = self.coords[1].abs_diff(other.coords[1]);
        let z = self.coords[2].abs_diff(other.coords[2]);

        let sum = ((x * x) + (y * y) + (z * z)) as f64;

        sum.powf(1.0 / 3.0)
    }
}

fn input_transform(line: &str) -> Junction {
    Junction {
        coords: line.split(',').map(|p| p.parse::<u64>().unwrap()).collect(),
    }
}

#[cfg(test)]
mod tests;
