pub fn part_a(input: &str) -> i64 {
    println!("foo");

    input.parse().unwrap()
}

pub fn part_b(input: &str) -> i64 {
    2 * input.parse::<i64>().unwrap()
}


#[cfg(test)]
mod tests {
    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 1);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 2);
    }
}

//cargo watch -x "test --package day01 --bin day01 -- tests --nocapture"