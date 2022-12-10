use derive_more::{Add, AddAssign, Constructor, Sub, SubAssign};
use std::{str::FromStr, fmt, collections::HashSet};

type ParsedTOut = Vec<(Point, i32)>;
type ParsedTIn = [(Point, i32)];
type ReturnT = i64;

#[derive(Copy, Clone, Debug, Constructor, Add, Sub, AddAssign, SubAssign, Hash, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct ParsePointError;

impl fmt::Display for ParsePointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parsing string to Point struct encountered error")
    }
}

impl Point {
    fn abs_check_above_one(&self) -> bool {
        self.x.abs() > 1 || self.y.abs() > 1
    }
    fn signum_sum(&mut self, other: Point) {
        self.x += other.x.signum();
        self.y += other.y.signum();
    }
}

impl FromStr for Point {
    type Err = ParsePointError;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let dir = line.split(' ').next().unwrap();
        match dir {
            "U" => Ok(Point::new(0, 1)),
            "R" => Ok(Point::new(1, 0)),
            "D" => Ok(Point::new(0, -1)),
            "L" => Ok(Point::new(-1, 0)),
            _ => Err(ParsePointError),
        }
    }
}

pub fn parse(input: &str) -> ParsedTOut {
    input
        .trim()
        .lines()
        .map(|f| (f.parse::<Point>().unwrap(), f.split_once(' ').unwrap().1.parse::<i32>().unwrap()))
        .collect::<ParsedTOut>()
}

fn solve<const N: usize>(val: &ParsedTIn) -> ReturnT {
    let mut rope = [Point::new(0,0); N];
    let mut visited: HashSet<Point> = HashSet::new();
    for step in val {
        for _ in 0..step.1 {
            rope[0] += step.0;
            for i in 1..N {
                let (h, t) = (rope[i-1], &mut rope[i]);
                let dp = h - *t;
                if dp.abs_check_above_one() {
                    t.signum_sum(dp);
                }
            }
            visited.insert(rope[N-1]);
        }
    }
    visited.len() as ReturnT
}

pub fn part_1(val: &ParsedTIn) -> ReturnT {
    solve::<2>(val)
}

pub fn part_2(val: &ParsedTIn) -> ReturnT {
    solve::<10>(val)
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1_example() {
        let parsed = super::parse(include_str!("input_example.txt"));
        assert_eq!(super::part_1(&parsed), 13);
    }

    #[test]
    fn part_1() {
        let parsed = super::parse(include_str!("input.txt"));
        assert_eq!(super::part_1(&parsed), 6503);
    }

    #[test]
    fn part_2_example() {
        let parsed = super::parse(include_str!("input_example.txt"));
        assert_eq!(super::part_2(&parsed), 1);
    }

    #[test]
    fn part_2_example2() {
        let parsed = super::parse(include_str!("input_example2.txt"));
        assert_eq!(super::part_2(&parsed), 36);
    }

    #[test]
    fn part_2() {
        let parsed = super::parse(include_str!("input.txt"));
        assert_eq!(super::part_2(&parsed), 2724);
    }
}
