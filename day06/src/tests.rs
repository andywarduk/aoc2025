use super::*;

const EXAMPLE1: &str = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

#[test]
fn test1() {
    let (numbers, operators) = input_transform1(EXAMPLE1);
    assert_eq!(part1(&numbers, &operators), 4277556);

    let (numbers, operators) = input_transform2(EXAMPLE1);
    println!("{:?}", numbers);
    assert_eq!(part2(&numbers, &operators), 3263827);
}
