use std::collections::HashSet;

fn letter_priority(letter: &char) -> i64 {
    if letter.is_uppercase() {
        *letter as i64 - 38
    }
    else {
        *letter as i64 - 96
    }
}

fn parse(input: &str) -> Vec<(&str, &str)> {
    input
        .trim()
        .lines()
        .map(|c| c.split_at(c.len()/2))
        .collect()
}

pub fn part_1(val: &[(&str, &str)]) -> i64 {
    let vector_of_hash_sets: Vec<(HashSet<char>, HashSet<char>)> = val
        .iter()
        .map(|f| (f.0.chars().collect(), f.1.chars().collect()))
        .collect();

    vector_of_hash_sets
        .iter()
        .map(|m| letter_priority(m.0.intersection(&m.1).next().unwrap()))
        .sum()
}

pub fn part_2(val: &[(&str, &str)]) -> i64 {
    let vector_of_str: Vec<String> = val
        .iter()
        .map(|c| (c.0.to_string() +  c.1))
        .collect();

    let vector_of_hashset: Vec<HashSet<char>> = vector_of_str
        .iter()
        .map(|f| (f.chars().collect()))
        .collect();
        
    let mut count: i64 = 0;
    assert!(val.len() % 3 == 0);

    let mut i = 0;
    while i < vector_of_hashset.len() {
        let a: &HashSet<char> = &vector_of_hashset[i+0];
        let b: &HashSet<char> = &vector_of_hashset[i+1];
        let c: &HashSet<char> = &vector_of_hashset[i+2];

        let sum: i64 = a
            .iter()
            .filter(|f| b.contains(*f) && c.contains(*f))
            .map(letter_priority)
            .sum();

        count += sum;

        i+=3;
    }

    count
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
