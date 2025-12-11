use std::error::Error;

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(9, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[Coord]) -> u64 {
    input.iter().enumerate().fold(0, |acc, (i, a)| {
        input.iter().skip(i + 1).fold(acc, |acc, b| {
            acc.max((a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1))
        })
    })
}

fn part2(input: &[Coord]) -> u64 {
    let mut areas = input
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            input
                .iter()
                .enumerate()
                .skip(i + 1)
                .map(move |(j, b)| (i, j, (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1)))
        })
        .collect::<Vec<_>>();

    areas.sort_by(|a, b| b.2.cmp(&a.2));

    for (a, b, area) in areas {
        let r1 = &input[a];
        let r2 = &input[b];

        let rmin = Coord {
            x: r1.x.min(r2.x),
            y: r1.y.min(r2.y),
        };

        let rmax = Coord {
            x: r1.x.max(r2.x),
            y: r1.y.max(r2.y),
        };

        let mut l1 = input.iter().next_back().unwrap();

        let found = input.iter().find(|l2| {
            // Check if line segment (l1, l2) intersects rectangle (rmin, rmax) anywhere except edges
            let intersects = line_segment_intersects_rect(l1, l2, &rmin, &rmax);

            l1 = l2;

            intersects
        });

        if found.is_none() {
            return area;
        }
    }

    0
}

// Line segment to rectangle intersection detection
fn line_segment_intersects_rect(l1: &Coord, l2: &Coord, rmin: &Coord, rmax: &Coord) -> bool {
    if l1.y == l2.y {
        // Horizontal line
        // Does this line potentially intersect on y?
        if l1.y > rmin.y && l1.y < rmax.y {
            // Check y intersection
            let min = l1.x.min(l2.x);
            let max = l1.x.max(l2.x);

            // Wholly inside, intersect left, or intersect right?
            if min > rmin.x && max < rmax.x
                || min < rmin.x && max > rmin.x
                || min < rmax.x && max > rmax.x
            {
                return true;
            }
        }
    } else if l1.x == l2.x {
        // Vertical line
        // Does this line potentially intersect on x?
        if l1.x > rmin.x && l1.x < rmax.x {
            // Check y intersection
            let min = l1.y.min(l2.y);
            let max = l1.y.max(l2.y);

            // Wholly inside, intersect top, or intersect bottom?
            if min > rmin.y && max < rmax.y
                || min < rmin.y && max > rmin.y
                || min < rmax.y && max > rmax.y
            {
                return true;
            }
        }
    } else {
        panic!()
    }

    false
}

// Input parsing

#[derive(Debug, Clone, Copy)]
struct Coord {
    x: i64,
    y: i64,
}

fn input_transform(line: &str) -> Coord {
    let (x, y) = line.split_once(',').unwrap();

    Coord {
        x: x.parse::<i64>().unwrap(),
        y: y.parse::<i64>().unwrap(),
    }
}

#[cfg(test)]
mod tests;
