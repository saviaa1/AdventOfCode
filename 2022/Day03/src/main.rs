use std::collections::HashSet;
use itertools::Itertools;

type ParsedTOut<'a> = Vec<(&'a str, &'a str)>;
type ParsedTIn<'a> = [(&'a str, &'a str)];
type ReturnT = i64;

fn letter_priority(letter: &char) -> i64 {
    if letter.is_uppercase() {
        *letter as i64 - 38
    }
    else {
        *letter as i64 - 96
    }
}

fn parse(input: &str) -> ParsedTOut {
    input
        .trim()
        .lines()
        .map(|c| c.split_at(c.len()/2))
        .collect()
}

pub fn part_1(val: &ParsedTIn) -> ReturnT {
    val
        .iter()
        .map(|f| (f.0.chars().collect::<HashSet<char>>(), f.1.chars().collect::<HashSet<char>>()))
        .map(|m| letter_priority(m.0.intersection(&m.1).next().unwrap()))
        .sum()
}

pub fn part_2(val: &ParsedTIn) -> ReturnT {
    assert!(val.len() % 3 == 0);

    val
        .iter()
        .map(|c| (c.0.to_string() +  c.1))
        .map(|f| (f.chars().collect::<HashSet<char>>()))
        .tuples()
        .map(|(a,b,c)| a.iter().filter(|s| b.contains(s) && c.contains(s)).map(letter_priority).sum::<i64>())
        .sum()
}

pub fn main() {
    let parsed = parse(include_str!("input.txt"));
    println!("2022 Day03");
    println!("Part 1: {}", part_1(&parsed));
    println!("Part 2: {}", part_2(&parsed));
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1_example() {
        let parsed = super::parse(include_str!("input_example.txt"));
        assert_eq!(super::part_1(&parsed), 157);
    }

    #[test]
    fn part_1() {
        let parsed = super::parse(include_str!("input.txt"));
        assert_eq!(super::part_1(&parsed), 7903);
    }

    #[test]
    fn part_2_example() {
        let parsed = super::parse(include_str!("input_example.txt"));
        assert_eq!(super::part_2(&parsed), 70);
    }

    #[test]
    fn part_2() {
        let parsed = super::parse(include_str!("input.txt"));
        assert_eq!(super::part_2(&parsed), 2548);
    }
}
