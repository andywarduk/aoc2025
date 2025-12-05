use super::*;

const EXAMPLE1: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

#[test]
fn test1() {
    let (ranges, ingredients) = input_transform(EXAMPLE1);
    assert_eq!(part1(&ranges, &ingredients), 3);
    assert_eq!(part2(&ranges), 14);
}
