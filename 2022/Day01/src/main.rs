use std::cmp::Reverse;

fn parse(input: &str) -> Vec<i64> {
    input
        .split("\r\n\r\n")
        .map(|c| c.lines().filter_map(|l| l.parse::<i64>().ok()).sum())
        .collect()
}

pub fn part_1(calories: &[i64]) -> i64 {
    *calories.iter().max().unwrap()
}

pub fn part_2(calories: &[i64]) -> i64 {
    let mut cal = calories.to_vec();
    cal.sort_by_key(|f| Reverse(*f));
    cal.iter()
        .take(3)
        .sum()
}

pub fn main() {
    let parsed = parse(include_str!("input.txt"));
    println!("{}", part_1(&parsed));
    println!("{}", part_2(&parsed));
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