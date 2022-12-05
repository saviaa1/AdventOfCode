use regex::Regex;

type ReturnType = String;
type Stack = Vec<Vec<char>>;
type Moves = Vec<(usize, usize, usize)>;

fn parse(input: &str) -> (Stack, Moves) {
    let (stack, moves): (&str, &str) = input
        .split_once("\r\n\r\n")
        .unwrap();

    let stack = stack
        .split('\n')
        .map(|f| f.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut t = vec![Vec::with_capacity(stack.len()); stack[0].len()];
    for r in stack {
        for i in 0..r.len() {
            t[i].push(r[i]);
        }
    }

    t.iter_mut().for_each(|f| f.reverse());

    let stack = t
        .into_iter()
        .map(|f| f.into_iter().filter(|g| g.is_ascii_alphabetic()).collect::<Vec<char>>())
        .filter(|f| !f.is_empty())
        .collect::<Vec<Vec<char>>>();

    let regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let moves = moves
        .trim()
        .split("\r\n")
        .map(|f| {
            let cap = regex.captures(f).unwrap();
            (cap[1].parse::<usize>().unwrap(), cap[2].parse::<usize>().unwrap() - 1, cap[3].parse::<usize>().unwrap() - 1)
        })
        .collect::<Vec<(usize, usize, usize)>>();

    (stack, moves)
}

pub fn part_1(stack: &Stack, val: &[(usize, usize, usize)]) -> ReturnType {
    let mut stack = stack.clone();
    for v in val {
        for _ in 1..=(v.0) {
            let a = stack[v.1].pop().unwrap();
            stack[v.2].push(a);
        }
    }

    let a = stack
        .iter_mut()
        .map(|f| f.pop().unwrap())
        .collect::<Vec<char>>();

    a.iter().collect()
}

pub fn part_2(stack: &Stack, val: &[(usize, usize, usize)]) -> ReturnType {
    let mut stack = stack.clone();
    for v in val {
        let mut temp_v: Vec<char> = Vec::new();
        for _ in 1..=(v.0) {
            let a = stack[v.1].pop().unwrap();
            temp_v.push(a);
        }
        temp_v.reverse();
        stack[v.2].extend(temp_v);
    }

    let a = stack
        .iter_mut()
        .map(|f| f.pop().unwrap())
        .collect::<Vec<char>>();

    a.iter().collect()
}

pub fn main() {
    let (stack, parsed) = parse(include_str!("input.txt"));
    println!("2022 Day05");
    println!("Part 1: {}", part_1(&stack, &parsed));
    println!("Part 2: {}", part_2(&stack, &parsed));
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1_example() {
        let (stack, parsed) = super::parse(include_str!("input_example.txt"));
        assert_eq!(super::part_1(&stack, &parsed), "CMZ");
    }

    #[test]
    fn part_1() {
        let (stack, parsed) = super::parse(include_str!("input.txt"));
        assert_eq!(super::part_1(&stack, &parsed), "SVFDLGLWV");
    }

    #[test]
    fn part_2_example() {
        let (stack, parsed) = super::parse(include_str!("input_example.txt"));
        assert_eq!(super::part_2(&stack, &parsed), "MCD");
    }

    #[test]
    fn part_2() {
        let (stack, parsed) = super::parse(include_str!("input.txt"));
        assert_eq!(super::part_2(&stack, &parsed), "DCVTCVPCL");
    }
}
