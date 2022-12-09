use day09::*;

pub fn main() {
  let parsed = parse(include_str!("../input.txt"));
  println!("2022 Day09");
  println!("Part 1: {}", part_1(&parsed));
  println!("Part 2: {}", part_2(&parsed));
}