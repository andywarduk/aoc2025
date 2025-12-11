use aoc::input::parse_test_vec;

use super::*;

const EXAMPLE1: &str = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

const EXAMPLE2: &str = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";

#[test]
fn test1() {
    let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
    let map = create_map(&input);
    assert_eq!(part1(&map), 5);

    let input = parse_test_vec(EXAMPLE2, input_transform).unwrap();
    let map = create_map(&input);
    assert_eq!(part2(&map), 2);
}
