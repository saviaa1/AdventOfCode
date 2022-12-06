use std::cmp::Reverse;

type ParsedTOut = Vec<i64>;
type ParsedTIn = [i64];
type ReturnT = i64;

pub fn parse(input: &str) -> ParsedTOut {
    input
        .split("\r\n\r\n")
        .map(|c| c.lines().filter_map(|l| l.parse::<i64>().ok()).sum())
        .collect()
}

pub fn part_1(calories: &ParsedTIn) -> ReturnT {
    *calories.iter().max().unwrap()
}

pub fn part_2(calories: &ParsedTIn) -> ReturnT {
    let mut cal = calories.to_vec();
    cal.sort_by_key(|f| Reverse(*f));
    cal.iter()
        .take(3)
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1_example() {
        let parsed = super::parse(include_str!("input_example.txt"));
        assert_eq!(super::part_1(&parsed), 24000);
    }

    #[test]
    fn part_1() {
        let parsed = super::parse(include_str!("input.txt"));
        assert_eq!(super::part_1(&parsed), 72070);
    }

    #[test]
    fn part_2_example() {
        let parsed = super::parse(include_str!("input_example.txt"));
        assert_eq!(super::part_2(&parsed), 45000);
    }

    #[test]
    fn part_2() {
        let parsed = super::parse(include_str!("input.txt"));
        assert_eq!(super::part_2(&parsed), 211805);
    }
}

//cargo watch -x "test --package day01 --bin day01 -- tests --nocapture"