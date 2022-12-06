type Vec4d = (i64, i64, i64, i64);
type ParsedTOut = Vec<Vec4d>;
type ParsedTIn = [Vec4d];
type ReturnT = i64;

fn parse(input: &str) -> ParsedTOut {
    input
        .trim()
        .lines()
        .map(|f| {
            let (x, y) = f.split_once(',').unwrap();
            let (a,b) = x.split_once('-').unwrap();
            let (c,d) = y.split_once('-').unwrap();
            (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap(), c.parse::<i64>().unwrap(), d.parse::<i64>().unwrap())
        }).collect()
}

/**
 * Check if (x0,y0) is fully inside (x1,y1) or wise versa.
 */
fn check_if_contains(vec: &Vec4d) -> bool {
    let l = vec.0..=vec.1;
    let r = vec.2..=vec.3;
    l.contains(r.start()) && l.contains(r.end()) || r.contains(l.start()) && r.contains(l.end())
}

/**
 * Check if (x0,y0) overlaps (x1,y1) or wise versa.
 */
fn check_if_overlaps(vec: &Vec4d) -> bool {
    let l = vec.0..=vec.1;
    let r = vec.2..=vec.3;
    l.contains(r.start()) || l.contains(r.end()) || r.contains(l.start()) || r.contains(l.end())
}


pub fn part_1(val: &ParsedTIn) -> ReturnT {
    val.iter().filter(|f| check_if_contains(f)).count() as i64
}

pub fn part_2(val: &ParsedTIn) -> ReturnT {
    val.iter().filter(|f| check_if_overlaps(f)).count() as i64
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
