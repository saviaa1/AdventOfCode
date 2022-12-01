fn parse_sums(input: &str) -> Vec<i64> {
    let mut calories: Vec<i64> = Vec::new();
    let mut sum: i64 = 0;
    for line in input.split('\n') {
        let trimmed_line: &str = line.trim();
        if !trimmed_line.is_empty() {
            sum += trimmed_line.parse::<i64>().unwrap();
        } else {
            calories.push(sum);
            sum = 0;
        }
    }
    calories
}

pub fn part_a(input: &str) -> i64 {
    let calories: Vec<i64> = parse_sums(input);
    *calories.iter().max().unwrap()
}

pub fn part_b(input: &str) -> i64 {
    let mut calories: Vec<i64> = parse_sums(input);
    calories.sort();

    let len = calories.len();

    calories[len-1] + calories[len-2] + calories[len-3]
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a_example() {
        assert_eq!(super::part_a(include_str!("input1.txt")), 24000);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 72070);
    }

    #[test]
    fn part_b_example() {
        assert_eq!(super::part_b(include_str!("input1.txt")), 45000);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 211805);
    }
}

//cargo watch -x "test --package day01 --bin day01 -- tests --nocapture"