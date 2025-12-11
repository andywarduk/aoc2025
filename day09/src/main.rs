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
        let p1 = &input[a];
        let p2 = &input[b];

        let pmin = Coord {
            x: p1.x.min(p2.x),
            y: p1.y.min(p2.y),
        };

        let pmax = Coord {
            x: p1.x.max(p2.x),
            y: p1.y.max(p2.y),
        };

        let mut t1 = input.iter().next_back().unwrap();

        let found = input.iter().find(|t2| {
            // Check if line segment (t1, t2) intersects rectangle (p1, p2)
            let intersects = line_segment_intersects_rect(t1, t2, &pmin, &pmax);

            t1 = t2;

            intersects
        });

        if found.is_none() {
            return area;
        }
    }

    0
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

// Line segment to rectangle intersection detection
#[allow(clippy::too_many_arguments)]
fn line_segment_intersects_rect(t1: &Coord, t2: &Coord, pmin: &Coord, pmax: &Coord) -> bool {
    // Check if either endpoint is inside the rectangle
    if (t1.x > pmin.x && t1.x < pmax.x && t1.y > pmin.y && t1.y < pmax.y)
        || (t2.x > pmin.x && t2.x < pmax.x && t2.y > pmin.y && t2.y < pmax.y)
    {
        return true;
    }

    // Check intersection with rectangle edges using Cohen-Sutherland algorithm
    let dx = t2.x - t1.x;
    let dy = t2.y - t1.y;

    // Check intersection with left and right edges
    if dx != 0 {
        let t_left = (pmin.x as f64 - t1.x as f64) / dx as f64;
        let t_right = (pmax.x as f64 - t1.x as f64) / dx as f64;

        for t in [t_left, t_right] {
            if t > 0.0 && t < 1.0 {
                let y = t1.y as f64 + t * dy as f64;
                if y > pmin.y as f64 && y < pmax.y as f64 {
                    return true;
                }
            }
        }
    }

    // Check intersection with top and bottom edges
    if dy != 0 {
        let t_bottom = (pmin.y as f64 - t1.y as f64) / dy as f64;
        let t_top = (pmax.y as f64 - t1.y as f64) / dy as f64;

        for t in [t_bottom, t_top] {
            if t > 0.0 && t < 1.0 {
                let x = t1.x as f64 + t * dx as f64;
                if x >= pmin.x as f64 && x <= pmax.x as f64 {
                    return true;
                }
            }
        }
    }

    // Check if line is axis-aligned and passes through rectangle
    if dx == 0 && t1.x > pmin.x && t1.x < pmax.x {
        let y_min = t1.y.min(t2.y);
        let y_max = t1.y.max(t2.y);
        if !(y_max < pmin.y || y_min > pmax.y) {
            return true;
        }
    }

    if dy == 0 && t1.y > pmin.y && t1.y < pmax.y {
        let x_min = t1.x.min(t2.x);
        let x_max = t1.x.max(t2.x);
        if !(x_max < pmin.x || x_min > pmax.x) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests;
