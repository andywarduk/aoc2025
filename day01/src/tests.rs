use aoc::input::parse_test_vec;

use super::*;

const EXAMPLE1: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

#[test]
fn test1() {
    let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
    assert_eq!(part1(&input), 3);
    assert_eq!(part2(&input), 6);
}

#[test]
fn test2() {
    let input = parse_test_vec("R1000", input_transform).unwrap();
    assert_eq!(part2(&input), 10);
}

#[test]
fn test3() {
    let input = parse_test_vec("R1050", input_transform).unwrap();
    assert_eq!(part2(&input), 11);
}

#[test]
fn test4() {
    let input = parse_test_vec(
        "L50
R100",
        input_transform,
    )
    .unwrap();
    assert_eq!(part2(&input), 2);
}
