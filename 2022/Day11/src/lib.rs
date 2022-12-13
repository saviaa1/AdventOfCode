use std::{fmt, str::FromStr, cmp::Reverse};
use regex::Regex;

type ParsedTOut = Vec<Monkey>;
type ParsedTIn = [Monkey];
type ReturnT = i64;

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Add(i64),
    Mult(i64),
    Square,
}

#[derive(Debug, Clone)]
pub struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    test: i64,
    result: [i64; 2],
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseLineError;

impl fmt::Display for ParseLineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parsing string to Line struct encountered error")
    }
}

impl FromStr for Monkey {
    type Err = ParseLineError;
    fn from_str(lines: &str) -> Result<Self, Self::Err> {
        //let monkey_r = Regex::new(r"Monkey (\d+):").unwrap();
        let items_r = Regex::new(r"  Starting items: ([0-9, ]+)").unwrap();
        let operation_r = Regex::new(r"  Operation: new = old (\*|\+) (\d+|old)").unwrap();
        let test_r = Regex::new(r"  Test: divisible by (\d+)").unwrap();
        let if_true_r = Regex::new(r"    If true: throw to monkey (\d+)").unwrap();
        let if_false_r = Regex::new(r"    If false: throw to monkey (\d+)").unwrap();

        let mut line = lines.trim().lines();

        line.next();

        /*let _id = monkey_r
            .captures(line.next().unwrap())
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<i64>()
            .unwrap();*/
        
        Ok(Self {
            items: {
                items_r
                    .captures(line.next().unwrap())
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .split(", ")
                    .map(|f| f.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>()
            },
            operation: {
                let operation_cap = operation_r
                    .captures(line.next().unwrap()).unwrap();
                let op = operation_cap.get(1).unwrap().as_str();
                let rop = operation_cap.get(2).unwrap().as_str();
                
                match rop {
                    "old" => Operation::Square,
                    b => match (op, b.parse::<i64>().unwrap()) {
                        ("+", n) => Operation::Add(n),
                        ("*", n) => Operation::Mult(n),
                        _ => unreachable!(),
                    }
                }
            },
            test: {
                test_r
                    .captures(line.next().unwrap())
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<i64>()
                    .unwrap()
            },
            result: {
                let if_true = if_true_r
                    .captures(line.next().unwrap())
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<i64>()
                    .unwrap();

                let if_false = if_false_r
                    .captures(line.next().unwrap())
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<i64>()
                    .unwrap();
                [if_true, if_false]
            },
        })
    }
}

pub fn parse(input: &str) -> ParsedTOut {
    input
        .trim()
        .split("\r\n\r\n")
        .map(|f| f.parse::<Monkey>().unwrap())
        .collect::<ParsedTOut>()
}

fn sim<const N: usize>(val: &ParsedTIn, f: impl Fn(i64) -> i64) -> ReturnT {
    let mut monkeys: Vec<Monkey> = val.to_vec();
    let mut inspections = vec![0; val.len()];
    for _ in 0..N {
        for i in 0..monkeys.len() {
            for item in monkeys[i].clone().items {
                let worry = match monkeys[i].operation {
                    Operation::Add(v) => f(item + v),
                    Operation::Mult(v) => f(item * v),
                    Operation::Square => f(item * item),
                };
                let a = if worry % monkeys[i].test == 0 {monkeys[i].result[0] as usize} else {monkeys[i].result[1] as usize};
                monkeys[a].items.push(worry);
            }
            inspections[i] += monkeys[i].items.len();
            monkeys[i].items.clear();
        }
    }
    inspections.sort_by_key(|&w| Reverse(w));
    (inspections[0] as i64) * (inspections[1] as i64)
}

pub fn part_1(val: &ParsedTIn) -> ReturnT {
    sim::<20>(val, |x| x/3)
}

pub fn part_2(val: &ParsedTIn) -> ReturnT {
    let modulus = val.iter().map(|m| m.test).product::<i64>();
    sim::<10000>(val, |x| x % modulus)
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1_example() {
        let parsed = super::parse(include_str!("input_example.txt"));
        assert_eq!(super::part_1(&parsed), 10605);
    }

    #[test]
    fn part_1() {
        let parsed = super::parse(include_str!("input.txt"));
        assert_eq!(super::part_1(&parsed), 110220);
    }

    #[test]
    fn part_2_example() {
        let parsed = super::parse(include_str!("input_example.txt"));
        assert_eq!(super::part_2(&parsed), 2713310158);
    }

    #[test]
    fn part_2() {
        let parsed = super::parse(include_str!("input.txt"));
        assert_eq!(super::part_2(&parsed), 19457438264);
    }
}
