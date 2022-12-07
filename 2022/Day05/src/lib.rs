use std::collections::VecDeque;
use regex::Regex;

type Stack = Vec<VecDeque<char>>;
type Moves = Vec<Move>;
type ParsedTOut = (Stack, Moves);
type ParsedTIn = (Stack, Moves);
type ReturnT = String;

pub struct Move {
    n: usize,
    from: usize,
    to: usize,
}

pub fn parse(input: &str) -> ParsedTOut {
    let (stack, moves): (&str, &str) = input
        .split_once("\r\n\r\n")
        .unwrap();


    let mut stacks = Vec::new();
    for line in stack.lines() {
        let chars: Vec<char> = line.trim_end().chars().collect();
        if chars[1] == '1' {
            break;
        }
        let n = (chars.len() + 1) / 4;
        for _ in stacks.len()..n {
            stacks.push(VecDeque::new());
        }
        
        stacks.iter_mut().enumerate().take(n).for_each(|f| {
            let pos_char = f.0 * 4 + 1;
            if chars[pos_char] != ' ' {
                f.1.push_front(chars[pos_char]);
            }
        });
    }

    let regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let moves = moves
        .trim()
        .split("\r\n")
        .map(|f| {
            let cap = regex.captures(f).unwrap();
            Move{n: cap[1].parse::<usize>().unwrap(), from: cap[2].parse::<usize>().unwrap() - 1, to: cap[3].parse::<usize>().unwrap() - 1}
        })
        .collect::<Moves>();

    (stacks, moves)
}

pub fn part_1(val: &ParsedTIn) -> ReturnT {
    let (stack, moves) = val;
    let mut stack = stack.clone();
    for m in moves {
        let a = stack.get_mut(m.from).unwrap();
        let mut b = a.split_off(a.len() - m.n);
        b.make_contiguous().reverse();
        stack[m.to].append(&mut b);
    }
    stack.iter().map_while(VecDeque::back).collect()
}

pub fn part_2(val: &ParsedTIn) -> ReturnT {
    let (stack, moves) = val;
    let mut stack = stack.clone();
    for m in moves {
        let a = stack.get_mut(m.from).unwrap();
        let mut b = a.split_off(a.len() - m.n);
        stack[m.to].append(&mut b);
    }
    stack.iter().map_while(VecDeque::back).collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1_example() {
        let parsed = super::parse(include_str!("input_example.txt"));
        assert_eq!(super::part_1(&parsed), "CMZ");
    }

    #[test]
    fn part_1() {
        let parsed = super::parse(include_str!("input.txt"));
        assert_eq!(super::part_1(&parsed), "SVFDLGLWV");
    }

    #[test]
    fn part_2_example() {
        let parsed = super::parse(include_str!("input_example.txt"));
        assert_eq!(super::part_2(&parsed), "MCD");
    }

    #[test]
    fn part_2() {
        let parsed = super::parse(include_str!("input.txt"));
        assert_eq!(super::part_2(&parsed), "DCVTCVPCL");
    }
    
    #[test]
    fn part_1_large() { // 1171s -> 79s (18 in release)
        let parsed = super::parse(include_str!("large_input.txt"));
        assert_eq!(super::part_1(&parsed), "GATHERING");
    }
    
    #[test]
    fn part_2_large() { //? Really long.
        let parsed = super::parse(include_str!("large_input.txt"));
        assert_eq!(super::part_1(&parsed), "GATHERING"); //DEVSCHUUR
    }
    
}
