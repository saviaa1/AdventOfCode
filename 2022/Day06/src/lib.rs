use std::collections::HashSet;

type ParsedTOut = Vec<char>;
type ParsedTIn = [char];
type ReturnT = i64;

pub fn parse(input: &str) -> ParsedTOut {
    input.trim().chars().collect::<ParsedTOut>()
}

pub fn part_1(val: &ParsedTIn) -> ReturnT {
    let mut count: i64 = 4;
    for window in  val.windows(4) {
        let hset: HashSet<&char> = HashSet::from_iter(window.iter());
        if hset.len() == 4 {
            return count;
        }
        count += 1;
    }
    panic!("Result not found!")
}

pub fn part_2(val: &ParsedTIn) -> ReturnT {
    let mut count: i64 = 14;
    for window in  val.windows(14) {
        let mut hset: HashSet<&char> = HashSet::new();
        for i in window.iter() {
            if !hset.insert(i) {
                break;
            }
        }
        if hset.len() == 14 {
            return count;
        }
        count += 1;
    }
    panic!("Result not found!")
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
