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

        let pxmin = p1.x.min(p2.x);
        let pxmax = p1.x.max(p2.x);

        let pymin = p1.y.min(p2.y);
        let pymax = p1.y.max(p2.y);

        let mut t1 = input.iter().next_back().unwrap();

        let found = input.iter().find(|t2| {
            // Check if line segment (t1, t2) intersects rectangle (p1, p2)
            let intersects =
                line_segment_intersects_rect(t1.x, t1.y, t2.x, t2.y, pxmin, pymin, pxmax, pymax);

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

#[derive(Debug)]
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
fn line_segment_intersects_rect(
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
    rx_min: i64,
    ry_min: i64,
    rx_max: i64,
    ry_max: i64,
) -> bool {
    // Check if either endpoint is inside the rectangle
    if (x1 > rx_min && x1 < rx_max && y1 > ry_min && y1 < ry_max)
        || (x2 > rx_min && x2 < rx_max && y2 > ry_min && y2 < ry_max)
    {
        return true;
    }

    // Check intersection with rectangle edges using Cohen-Sutherland algorithm
    let dx = x2 - x1;
    let dy = y2 - y1;

    // Check intersection with left and right edges
    if dx != 0 {
        let t_left = (rx_min as f64 - x1 as f64) / dx as f64;
        let t_right = (rx_max as f64 - x1 as f64) / dx as f64;

        for t in [t_left, t_right] {
            if t > 0.0 && t < 1.0 {
                let y = y1 as f64 + t * dy as f64;
                if y > ry_min as f64 && y < ry_max as f64 {
                    return true;
                }
            }
        }
    }

    // Check intersection with top and bottom edges
    if dy != 0 {
        let t_bottom = (ry_min as f64 - y1 as f64) / dy as f64;
        let t_top = (ry_max as f64 - y1 as f64) / dy as f64;

        for t in [t_bottom, t_top] {
            if t > 0.0 && t < 1.0 {
                let x = x1 as f64 + t * dx as f64;
                if x >= rx_min as f64 && x <= rx_max as f64 {
                    return true;
                }
            }
        }
    }

    // Check if line is axis-aligned and passes through rectangle
    if dx == 0 && x1 > rx_min && x1 < rx_max {
        let y_min = y1.min(y2);
        let y_max = y1.max(y2);
        if !(y_max < ry_min || y_min > ry_max) {
            return true;
        }
    }

    if dy == 0 && y1 > ry_min && y1 < ry_max {
        let x_min = x1.min(x2);
        let x_max = x1.max(x2);
        if !(x_max < rx_min || x_min > rx_max) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests;
