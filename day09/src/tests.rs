use aoc::input::parse_test_vec;

use super::*;

const EXAMPLE1: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

#[test]
fn test1() {
    let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
    assert_eq!(part1(&input), 50);
    assert_eq!(part2(&input), 24);
}
