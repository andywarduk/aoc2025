use std::error::Error;

use aoc::{gif::Gif, input::parse_input_vec};

const CELL_SIZE: usize = 5;
const COL_GRADES: u8 = 8;
const DELAY_SCALE: u16 = 75;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(4, input_transform)?;

    // Run parts
    part2(&input, "vis/day04.gif")?;

    Ok(())
}

fn part2(input: &[InputEnt], file: &str) -> Result<(), Box<dyn Error>> {
    let mut board = input.to_vec();

    let w = board[0].len();
    let h = board.len();

    // Build the palette
    let mut palette = Vec::new();

    palette.push([0, 0, 0]);
    palette.push([0xff, 0xff, 0xff]);

    for i in 0..COL_GRADES {
        let byte = ((0xff * ((i as u16) + 1)) / COL_GRADES as u16) as u8;
        println!("{byte}");
        palette.push([byte, 0, byte / 2]);
    }

    // Calculate dimensions
    let gw = (input[0].len() * CELL_SIZE) as u16;
    let gh = (input.len() * CELL_SIZE) as u16;

    // Create the gif
    let mut gif = Gif::new(file, &palette, gw, gh, 1, 1)?;

    loop {
        let mut next_board = board.clone();
        let mut this_removed = 0;

        for y in 0..h {
            for x in 0..w {
                if board[y][x] == State::Paper && adjacent_count(&board, x, y) < 4 {
                    next_board[y][x] = State::Empty;
                    this_removed += 1;
                }
            }
        }

        if this_removed == 0 {
            break;
        }

        draw_board(&mut gif, &board, &next_board, this_removed / DELAY_SCALE)?;

        board = next_board;
        next_board = board.clone();
    }

    gif.delay(100)?;

    Ok(())
}

const SPRITE: [[u8; CELL_SIZE]; CELL_SIZE] = [
    [0, 1, 1, 1, 0],
    [1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1],
    [0, 1, 1, 1, 0],
];

fn draw_board(
    gif: &mut Gif,
    old_board: &[Vec<State>],
    new_board: &[Vec<State>],
    delay: u16,
) -> Result<(), Box<dyn Error>> {
    for col in (0..COL_GRADES).rev() {
        // Create the frame
        let mut frame_data = gif.empty_frame();

        for y in 0..(new_board.len()) {
            for x in 0..(new_board[y].len()) {
                let use_col = match (&old_board[y][x], &new_board[y][x]) {
                    (State::Empty, State::Empty) => continue,
                    (State::Paper, State::Empty) => col + 2,
                    (State::Paper, State::Paper) => 1,
                    _ => unreachable!(),
                };

                let gx = x * CELL_SIZE;
                let gy = y * CELL_SIZE;

                for oy in 0..CELL_SIZE {
                    for ox in 0..CELL_SIZE {
                        if SPRITE[oy][ox] == 1 {
                            frame_data[gy + oy][gx + ox] = use_col;
                        }
                    }
                }
            }
        }

        // Output the frame
        gif.draw_frame(frame_data, delay)?;
    }

    Ok(())
}

fn adjacent_count(input: &[InputEnt], x: usize, y: usize) -> usize {
    adjacent_coords(input, x, y)
        .filter(|&(x, y)| input[y][x] == State::Paper)
        .count()
}

fn adjacent_coords(input: &[InputEnt], x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    let x = x as isize;
    let y = y as isize;

    let w = input[0].len() as isize;
    let h = input.len() as isize;

    ((y - 1)..=(y + 1)).flat_map(move |ty| {
        ((x - 1)..=(x + 1)).filter_map(move |tx| {
            if ty >= 0 && ty < h && tx >= 0 && tx < w && !(tx == x && ty == y) {
                Some((tx as usize, ty as usize))
            } else {
                None
            }
        })
    })
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
