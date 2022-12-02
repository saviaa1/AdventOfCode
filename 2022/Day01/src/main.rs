use std::cmp::Reverse;

fn parse(input: &str) -> Vec<i64> {
    input
        .split("\r\n\r\n")
        .map(|c| c.lines().filter_map(|l| l.parse::<i64>().ok()).sum())
        .collect()
}

pub fn part_a(calories: &[i64]) -> i64 {
    *calories.iter().max().unwrap()
}

pub fn part_b(calories: &[i64]) -> i64 {
    let mut cal = calories.to_vec();
    cal.sort_by_key(|f| Reverse(*f));
    cal.iter()
        .take(3)
        .sum()
}

pub fn main() {
    let parsed = parse(include_str!("input.txt"));
    println!("{}", part_a(&parsed));
    println!("{}", part_b(&parsed));
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a_example() {
        let parsed = super::parse(include_str!("input1.txt"));
        assert_eq!(super::part_a(&parsed), 24000);
    }

    #[test]
    fn part_a() {
        let parsed = super::parse(include_str!("input.txt"));
        assert_eq!(super::part_a(&parsed), 72070);
    }

    #[test]
    fn part_b_example() {
        let parsed = super::parse(include_str!("input1.txt"));
        assert_eq!(super::part_b(&parsed), 45000);
    }

    #[test]
    fn part_b() {
        let parsed = super::parse(include_str!("input.txt"));
        assert_eq!(super::part_b(&parsed), 211805);
    }
}

//cargo watch -x "test --package day01 --bin day01 -- tests --nocapture"