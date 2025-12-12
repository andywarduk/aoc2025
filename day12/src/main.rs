use std::{collections::HashSet, error::Error};

use aoc::input::parse_input;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let (shapes, boards) = parse_input(12, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&shapes, &boards, false));

    Ok(())
}

fn part1(shapes: &[Shape], boards: &[Board], prove: bool) -> u64 {
    boards
        .iter()
        .filter_map(|board| {
            let board_area = board.width as u16 * board.height as u16;
            let shape_area: u16 = board
                .shapes
                .iter()
                .enumerate()
                .map(|(shape_idx, count)| *count as u16 * shapes[shape_idx].area as u16)
                .sum();

            if shape_area > board_area {
                None
            } else if prove {
                // Packer
                let available = board
                    .shapes
                    .iter()
                    .enumerate()
                    .flat_map(|(shape_no, num)| vec![shape_no; *num])
                    .collect::<Vec<_>>();

                let board_lines = vec![0; board.height as usize];
                let shapes_tried = HashSet::new();

                println!("Fitting {}x{}", board.width, board.height);

                if fit_board(
                    board_lines,
                    board.width as usize,
                    shapes,
                    available,
                    shapes_tried,
                ) {
                    Some(())
                } else {
                    None
                }
            } else {
                Some(())
            }
        })
        .count() as u64
}

fn fit_board(
    board: Vec<u64>,
    width: usize,
    shapes: &[Shape],
    available: Vec<usize>,
    mut shapes_tried: HashSet<(usize, usize, usize)>,
) -> bool {
    let height = board.len();

    for y in 0..height - 2 {
        for x in 0..width - 2 {
            let shift = (width - x) - 3;

            for (shape_idx, s) in available.iter().enumerate() {
                // Already tried this shape here?
                if shapes_tried.contains(&(x, y, *s)) {
                    continue;
                }

                shapes_tried.insert((x, y, *s));

                // Get shape
                let shape = &shapes[*s];

                // Iterate configurations
                for config in shape.masks.iter() {
                    // Does the shape fit here?
                    if config
                        .iter()
                        .zip(board.iter().skip(y))
                        .all(|(mask, board_line)| {
                            let test_mask = (*mask as u64) << shift;

                            board_line & test_mask == 0
                        })
                    {
                        // Yes
                        let mut next_available = available.clone();

                        next_available.swap_remove(shape_idx);

                        if next_available.is_empty() {
                            return true;
                        } else {
                            let mut next_board = board.clone();

                            config.iter().zip(next_board.iter_mut().skip(y)).for_each(
                                |(mask, board_line)| {
                                    let mask = (*mask as u64) << shift;
                                    *board_line |= mask;
                                },
                            );

                            if fit_board(
                                next_board,
                                width,
                                shapes,
                                next_available,
                                shapes_tried.clone(),
                            ) {
                                return true;
                            }
                        }
                    }
                }
            }
        }
    }

    false
}

// Input parsing

struct Shape {
    masks: Vec<[u8; 3]>,
    area: u8,
}

struct Board {
    width: u8,
    height: u8,
    shapes: Vec<usize>,
}

fn input_transform(input: &str) -> (Vec<Shape>, Vec<Board>) {
    let mut shapes = Vec::new();
    let mut boards = Vec::new();

    for section in input.split("\n\n") {
        if section.lines().next().unwrap().ends_with(':') {
            // Shape
            let chars: Vec<Vec<char>> = section
                .lines()
                .skip(1)
                .map(|line| line.chars().collect())
                .collect();

            shapes.push(build_shape(chars));
        } else {
            // Boards
            boards = section
                .lines()
                .map(|line| {
                    let (dim, shapes) = line.split_once(':').unwrap();

                    let (w, h) = dim.split_once('x').unwrap();

                    let shapes = shapes
                        .trim()
                        .split(' ')
                        .map(|shape| shape.parse::<usize>().unwrap())
                        .collect::<Vec<_>>();

                    Board {
                        width: w.parse::<u8>().unwrap(),
                        height: h.parse::<u8>().unwrap(),
                        shapes,
                    }
                })
                .collect();
        }
    }

    (shapes, boards)
}

fn build_shape(chars: Vec<Vec<char>>) -> Shape {
    let mut xi_yi = [0; 3];
    let mut xi_yd = [0; 3];
    let mut xd_yi = [0; 3];
    let mut xd_yd = [0; 3];

    let mut yi_xi = [0; 3];
    let mut yd_xi = [0; 3];
    let mut yi_xd = [0; 3];
    let mut yd_xd = [0; 3];

    let mut area = 0;

    for xi in 0..3 {
        let xd = 2 - xi;

        for yi in 0..3 {
            let yd = 2 - yi;

            if chars[yi][xi] == '#' {
                area += 1;

                xi_yi[yi] |= 1 << xi;
                xi_yd[yd] |= 1 << xi;
                xd_yi[yi] |= 1 << xd;
                xd_yd[yd] |= 1 << xd;

                yi_xi[xi] |= 1 << yi;
                yd_xi[xi] |= 1 << yd;
                yi_xd[xd] |= 1 << yi;
                yd_xd[xd] |= 1 << yd;
            }
        }
    }

    let mut masks = Vec::new();

    check_unique(&mut masks, xi_yi);
    check_unique(&mut masks, xi_yd);
    check_unique(&mut masks, xd_yi);
    check_unique(&mut masks, xd_yd);
    check_unique(&mut masks, yi_xi);
    check_unique(&mut masks, yd_xi);
    check_unique(&mut masks, yi_xd);
    check_unique(&mut masks, yd_xd);

    Shape { masks, area }
}

fn check_unique(masks: &mut Vec<[u8; 3]>, mask: [u8; 3]) {
    for exist in masks.iter() {
        if exist.iter().zip(mask.iter()).all(|(a, b)| *a == *b) {
            return;
        }
    }

    masks.push(mask)
}

#[cfg(test)]
mod tests;
