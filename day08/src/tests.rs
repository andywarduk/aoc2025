use aoc::input::parse_test_vec;

use super::*;

const EXAMPLE1: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

#[test]
fn test1() {
    let junctions = parse_test_vec(EXAMPLE1, input_transform).unwrap();

    let edges = build_edges(&junctions);

    assert_eq!(part1(&junctions, &edges, 10), 40);
    assert_eq!(part2(&junctions, &edges,), 25272);
}
