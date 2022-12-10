use std::{fmt, str::FromStr};

type ParsedTOut = Vec<Instruction>;
type ParsedTIn = [Instruction];
type ReturnT = i64;

#[derive(Debug, Clone)]
pub enum Instruction {
    Noop,
    Addx(i32)
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseLineError;

impl fmt::Display for ParseLineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parsing string to Line struct encountered error")
    }
}

impl FromStr for Instruction {
    type Err = ParseLineError;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut line = line.split(' ');
        let first = line.next().unwrap();
        match first {
            "noop" => Ok(Instruction::Noop),
            "addx" => Ok(Instruction::Addx(line.next().unwrap().parse().unwrap())),
            _ => Err(ParseLineError),
        }
    }
}

pub fn parse(input: &str) -> ParsedTOut {
    input
        .trim()
        .lines()
        .map(|f| f.parse().unwrap())
        .collect::<ParsedTOut>()
}

fn check_cycle(x: i32, c: i32) -> i32 {
    if (c+20) % 40 == 0 && c <= 220 {
        return x * c
    }
    0
}

pub fn part_1(val: &ParsedTIn) -> ReturnT {
    let mut cycle: i32 = 1;
    let mut x_reg: i32 = 1;
    let mut sum: i32 = 0;

    for inst in val {
        match inst {
            Instruction::Noop => cycle += 1,
            Instruction::Addx(x) => {
                cycle += 1;
                sum += check_cycle(x_reg, cycle);
                cycle += 1;
                x_reg += x;
            }
        }
        sum += check_cycle(x_reg, cycle);
    }
    sum as i64
}

#[derive(Debug, PartialEq, Eq)]
struct Screen {
    screen: String,
}

impl Screen {
    fn draw(&mut self, c: i32, reg: i32) {
        if (reg - ((c-1) % 40)).abs() < 2 {
            self.screen.push('#');
        } else {
            self.screen.push(' ');
        }
        if c % 40 == 0 {
            self.screen.push('\n');
        }
    }
}

pub fn part_2(val: &ParsedTIn) -> String {
    let mut cycle: i32 = 1;
    let mut x_reg: i32 = 1;
    let mut screen: Screen = Screen{screen: String::new()};

    for inst in val {
        match inst {
            Instruction::Noop => {
                screen.draw(cycle, x_reg);
                cycle += 1;
            },
            Instruction::Addx(x) => {
                screen.draw(cycle, x_reg);
                cycle += 1;
                screen.draw(cycle, x_reg);
                cycle += 1;
                x_reg += x;
            }
        }
    }
    screen.screen
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1_example() {
        let parsed = super::parse(include_str!("input_example.txt"));
        assert_eq!(super::part_1(&parsed), 13140);
    }

    #[test]
    fn part_1() {
        let parsed = super::parse(include_str!("input.txt"));
        assert_eq!(super::part_1(&parsed), 13820);
    }

    #[test]
    fn part_2_example() {
        let parsed = super::parse(include_str!("input_example.txt"));
        let should_be: String = String::from("##  ##  ##  ##  ##  ##  ##  ##  ##  ##  \n###   ###   ###   ###   ###   ###   ### \n####    ####    ####    ####    ####    \n#####     #####     #####     #####     \n######      ######      ######      ####\n#######       #######       #######     \n");
        assert_eq!(super::part_2(&parsed), should_be);
    }

    #[test]
    fn part_2() {
        let parsed = super::parse(include_str!("input.txt"));
        let should_be: String = String::from("#### #  #  ##  ###  #  #  ##  ###  #  # \n   # # #  #  # #  # # #  #  # #  # # #  \n  #  ##   #    #  # ##   #    #  # ##   \n #   # #  # ## ###  # #  # ## ###  # #  \n#    # #  #  # # #  # #  #  # # #  # #  \n#### #  #  ### #  # #  #  ### #  # #  # \n");
        assert_eq!(super::part_2(&parsed), should_be);
    }
}
