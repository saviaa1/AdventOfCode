use std::collections::HashSet;

type ParsedTOut = Vec<char>;
type ParsedTIn = [char];
type ReturnT = i64;

fn parse(input: &str) -> ParsedTOut {
    input.trim().chars().collect::<ParsedTOut>()
}

pub fn part_1(val: &ParsedTIn) -> ReturnT {
    for f in  val.iter().enumerate().collect::<Vec<(usize, &char)>>().windows(4) {
        let mut hset: HashSet<&char> = HashSet::new();
        if hset.insert(f[0].1) && hset.insert(f[1].1) && hset.insert(f[2].1) && hset.insert(f[3].1) {
            return (f[3].0 + 1) as ReturnT
        }
    }
    panic!("Result not found!")
}

pub fn part_2(val: &ParsedTIn) -> ReturnT {
    for f in  val.iter().enumerate().collect::<Vec<(usize, &char)>>().windows(14) {
        let mut hset: HashSet<&char> = HashSet::new();
        for i in 0..=13 {
            if !hset.insert(f[i].1) {
                break;
            }
        }
        if hset.len() == 14 {
            return (f[13].0 + 1) as ReturnT
        }
    }
    panic!("Result not found!")
}

pub fn main() {
    let parsed = parse(include_str!("input.txt"));
    println!("2022 Day06");
    println!("Part 1: {}", part_1(&parsed));
    println!("Part 2: {}", part_2(&parsed));
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1_example() {
        let parsed = super::parse(include_str!("input_example1.txt"));
        assert_eq!(super::part_1(&parsed), 7);
    }

    #[test]
    fn part_1_example2() {
        let parsed = super::parse(include_str!("input_example2.txt"));
        assert_eq!(super::part_1(&parsed), 5);
    }

    #[test]
    fn part_1_example3() {
        let parsed = super::parse(include_str!("input_example3.txt"));
        assert_eq!(super::part_1(&parsed), 6);
    }

    #[test]
    fn part_1_example4() {
        let parsed = super::parse(include_str!("input_example4.txt"));
        assert_eq!(super::part_1(&parsed), 10);
    }

    #[test]
    fn part_1_example5() {
        let parsed = super::parse(include_str!("input_example5.txt"));
        assert_eq!(super::part_1(&parsed), 11);
    }

    #[test]
    fn part_1() {
        let parsed = super::parse(include_str!("input.txt"));
        assert_eq!(super::part_1(&parsed), 1760);
    }

    #[test]
    fn part_2_example() {
        let parsed = super::parse(include_str!("input_example6.txt"));
        assert_eq!(super::part_2(&parsed), 19);
    }

    #[test]
    fn part_2_example2() {
        let parsed = super::parse(include_str!("input_example7.txt"));
        assert_eq!(super::part_2(&parsed), 23);
    }

    #[test]
    fn part_2_example3() {
        let parsed = super::parse(include_str!("input_example8.txt"));
        assert_eq!(super::part_2(&parsed), 23);
    }

    #[test]
    fn part_2_example4() {
        let parsed = super::parse(include_str!("input_example9.txt"));
        assert_eq!(super::part_2(&parsed), 29);
    }

    #[test]
    fn part_2_example5() {
        let parsed = super::parse(include_str!("input_example10.txt"));
        assert_eq!(super::part_2(&parsed), 26);
    }

    #[test]
    fn part_2() {
        let parsed = super::parse(include_str!("input.txt"));
        assert_eq!(super::part_2(&parsed), 2974);
    }
}
