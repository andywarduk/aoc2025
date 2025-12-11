use std::{
    cmp::{max, min},
    error::Error,
    fs::File,
    io::Write,
};

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(9, input_transform)?;

    // Run part 1
    let (p1_1, p1_2) = part1(&input);

    // Run part 2
    let (p2_1, p2_2) = part2(&input);

    // Draw
    draw(&input, p1_1, p1_2, p2_1, p2_2, "vis/day09.svg")?;

    Ok(())
}

fn part1(input: &[Coord]) -> (Coord, Coord) {
    let (_, p1, p2) = input
        .iter()
        .enumerate()
        .fold((0, None, None), |acc, (i, a)| {
            input
                .iter()
                .skip(i + 1)
                .fold(acc, |(biggest_area, p1, p2), b| {
                    let area = (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1);

                    if area > biggest_area {
                        (area, Some(a), Some(b))
                    } else {
                        (biggest_area, p1, p2)
                    }
                })
        });

    (*p1.unwrap(), *p2.unwrap())
}

fn part2(input: &[Coord]) -> (Coord, Coord) {
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

    for (a, b, _area) in areas {
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
            return (*r1, *r2);
        }
    }

    panic!();
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

fn draw(
    coords: &[Coord],
    p1_1: Coord,
    p1_2: Coord,
    p2_1: Coord,
    p2_2: Coord,
    file: &str,
) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(file)?;

    let mut min_x = i64::MAX;
    let mut min_y = i64::MAX;
    let mut max_x = i64::MIN;
    let mut max_y = i64::MIN;

    for coord in coords {
        min_x = min(min_x, coord.x);
        min_y = min(min_y, coord.y);
        max_x = max(max_x, coord.x);
        max_y = max(max_y, coord.y);
    }

    let y_height = max_y - min_y;
    let x_width = max_x - min_x;

    let height = 900;
    let ratio = height as f64 / y_height as f64;
    let width = (x_width as f64 * ratio) as i32;

    file.write_fmt(format_args!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>\n"
    ))?;
    file.write_fmt(format_args!("<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">\n"))?;

    file.write_fmt(format_args!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{width}\" height=\"{height}\" viewBox=\"{min_x} {min_y} {x_width} {y_height}\">\n"
    ))?;

    let mut iter = coords.iter();

    let coord = iter.next().unwrap();

    file.write_fmt(format_args!("<path d=\"M {} {}", coord.x, coord.y))?;

    for coord in iter {
        file.write_fmt(format_args!(" L {} {}", coord.x, coord.y))?;
    }

    file.write_fmt(format_args!(
        "Z \" stroke=\"red\" stroke-width=\"0.15%\" fill=\"white\"/>"
    ))?;

    draw_rect(&mut file, p1_1, p1_2, "green")?;

    draw_rect(&mut file, p2_1, p2_2, "blue")?;

    file.write_fmt(format_args!("</svg>\n"))?;

    Ok(())
}

fn draw_rect(file: &mut File, p1: Coord, p2: Coord, colour: &str) -> Result<(), Box<dyn Error>> {
    file.write_fmt(format_args!("<path d=\"M {} {}", p1.x, p1.y))?;
    file.write_fmt(format_args!(" L {} {}", p2.x, p1.y))?;
    file.write_fmt(format_args!(" L {} {}", p2.x, p2.y))?;
    file.write_fmt(format_args!(" L {} {}", p1.x, p2.y))?;
    file.write_fmt(format_args!(
        "Z \" stroke=\"{colour}\" stroke-width=\"0.15%\" fill=\"none\"/>"
    ))?;

    Ok(())
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
