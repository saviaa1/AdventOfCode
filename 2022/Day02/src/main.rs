fn parse(_input: &str) -> Vec<i64> {
    let res: Vec<i64> = Vec::new();
    res
}

pub fn part_a(calories: &[i64]) -> i64 {
    (calories.len() as i64) + 1
}

pub fn part_b(calories: &[i64]) -> i64 {
     (calories.len() as i64) + 2
}

pub fn main() {
    let parsed = parse(include_str!("input.txt"));
    println!("2022 Day02");
    println!("Part 1: {}", part_a(&parsed));
    println!("Part 2: {}", part_b(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_example() {
        let parsed = parse(include_str!("input.txt"));
        assert_eq!(part_a(&parsed), 1);
    }

    #[test]
    fn part_b_example() {
        let parsed = parse(include_str!("input.txt"));
        assert_eq!(part_b(&parsed), 2);
    }
}
