use aoc::input::parse_test_vec;

use super::*;

const EXAMPLE1: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111";

#[test]
fn test1() {
    let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
    assert_eq!(part1(&input), 357);
    assert_eq!(part2(&input), 3121910778619);
}
