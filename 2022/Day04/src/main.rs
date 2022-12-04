type Parsed = Vec<(i64, i64, i64, i64)>;

fn parse(input: &str) -> Parsed {
    input
        .trim()
        .lines()
        .map(|f| f.trim().split_once(',').unwrap())
        .map(|f| (f.0.split_once('-').unwrap(), f.1.split_once('-').unwrap()))
        .map(|f| (f.0.0.parse::<i64>().unwrap(), f.0.1.parse::<i64>().unwrap(), f.1.0.parse::<i64>().unwrap(), f.1.1.parse::<i64>().unwrap()))
        .collect()
}

/**
 * Check if (x0,y0) is inside (x1,y1) or wise versa.
 */
fn check_if_contains(x0: i64, y0: i64, x1: i64, y1: i64) -> bool {
    x0 >= x1 && x0 <= y1 && y0 >= x1 && y0 <= y1 || x1 >= x0 && x1 <= y0 && y1 >= x0 && y1 <= y0
}

/**
 * Check if (x0,y0) overlaps (x1,y1) or wise versa.
 */
fn check_if_overlaps(x0: i64, y0: i64, x1: i64, y1: i64) -> bool {
    x0 >= x1 && x0 <= y1 || y0 >= x1 && y0 <= y1 || x1 >= x0 && x1 <= y0 || y1 >= x0 && y1 <= y0
}


pub fn part_1(val: &Parsed) -> i64 {
    val.iter().filter(|f| check_if_contains(f.0, f.1, f.2, f.3)).count() as i64
}

pub fn part_2(val: &Parsed) -> i64 {
    val.iter().filter(|f| check_if_overlaps(f.0, f.1, f.2, f.3)).count() as i64
}

pub fn main() {
    let parsed = parse(include_str!("input.txt"));
    println!("2022 Day04");
    println!("Part 1: {}", part_1(&parsed));
    println!("Part 2: {}", part_2(&parsed));
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1_example() {
        let parsed = super::parse(include_str!("input_example.txt"));
        assert_eq!(super::part_1(&parsed), 2);
    }

    #[test]
    fn part_1() {
        let parsed = super::parse(include_str!("input.txt"));
        assert_eq!(super::part_1(&parsed), 431);
    }

    #[test]
    fn part_2_example() {
        let parsed = super::parse(include_str!("input_example.txt"));
        assert_eq!(super::part_2(&parsed), 4);
    }

    #[test]
    fn part_2() {
        let parsed = super::parse(include_str!("input.txt"));
        assert_eq!(super::part_2(&parsed), 823);
    }
}
