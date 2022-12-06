type ParsedTOut<'a> = Vec<&'a str>;
type ParsedTIn<'a> = [&'a str];
type ReturnT = i64;

fn parse(_input: &str) -> ParsedTOut {
    unimplemented!();
}

pub fn part_1(_val: &ParsedTIn) -> ReturnT {
    unimplemented!();
}

pub fn part_2(_val: &ParsedTIn) -> ReturnT {
    unimplemented!();
}

pub fn main() {
    let parsed = parse(include_str!("input.txt"));
    println!("2022 DayXX");
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
