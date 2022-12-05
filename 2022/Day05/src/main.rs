type ReturnType = String;

fn stack_input_example() -> Vec<Vec<char>> {
    let mut a = vec![
        vec!['N', 'Z'],
        vec!['D', 'C', 'M'],
        vec!['P']
    ];

    for v in a.iter_mut() {
        v.reverse();
    }

    a
}

fn stack_input() -> Vec<Vec<char>> {
    let mut a = vec![
        vec!['N', 'Q', 'L', 'S', 'C', 'Z', 'P', 'T'],
        vec!['G', 'C', 'H', 'V', 'T', 'P', 'L'],
        vec!['F', 'Z', 'C', 'D'],
        vec!['C', 'V', 'M', 'L', 'D', 'T', 'W', 'G'],
        vec!['C', 'W', 'P'],
        vec!['Z', 'S', 'T', 'C', 'D', 'J', 'F', 'P'],
        vec!['D', 'B', 'G', 'W', 'V'],
        vec!['W', 'H', 'Q', 'S', 'J', 'N'],
        vec!['V', 'L', 'S', 'F', 'Q', 'C', 'R']
    ];

    for v in a.iter_mut() {
        v.reverse();
    }

    a
}


fn parse(input: &str) -> Vec<(usize, usize, usize)> {
    let (_, moves): (&str, &str) = input
        .split_once("\r\n\r\n")
        .unwrap();

    let moves = moves
        .trim()
        .split("\r\n")
        .map(|f| {
            let s = f.split(' ').collect::<Vec<&str>>();
            (s[1].parse::<usize>().unwrap(), s[3].parse::<usize>().unwrap(), s[5].parse::<usize>().unwrap())
        })
        .collect::<Vec<(usize, usize, usize)>>();

    moves
}

pub fn part_1(val: &[(usize, usize, usize)], stack: Vec<Vec<char>>) -> ReturnType {
    let mut stack = stack;
    for v in val {
        //move 1 from 2 to 1
        for _ in 1..=(v.0) {
            let a = stack[v.1-1].pop().unwrap();
            stack[v.2-1].push(a);
        }
    }

    let a = stack
        .iter_mut()
        .map(|f| f.pop().unwrap())
        .collect::<Vec<char>>();

    a.iter().collect()
}

pub fn part_2(val: &[(usize, usize, usize)], stack: Vec<Vec<char>>) -> ReturnType {
    let mut stack = stack;
    for v in val {
        //move 1 from 2 to 1
        let mut temp_v: Vec<char> = Vec::new();
        for _ in 1..=(v.0) {
            let a = stack[v.1-1].pop().unwrap();
            temp_v.push(a);
        }
        temp_v.reverse();
        stack[v.2-1].extend(temp_v);
    }

    let a = stack
        .iter_mut()
        .map(|f| f.pop().unwrap())
        .collect::<Vec<char>>();

    a.iter().collect()
}

pub fn main() {
    let parsed = parse(include_str!("input.txt"));
    println!("2022 Day05");
    println!("Part 1: {}", part_1(&parsed, stack_input()));
    println!("Part 2: {}", part_2(&parsed, stack_input()));
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1_example() {
        let parsed = super::parse(include_str!("input_example.txt"));
        assert_eq!(super::part_1(&parsed, super::stack_input_example()), "CMZ");
    }

    #[test]
    fn part_1() {
        let parsed = super::parse(include_str!("input.txt"));
        assert_eq!(super::part_1(&parsed, super::stack_input()), "SVFDLGLWV");
    }

    #[test]
    fn part_2_example() {
        let parsed = super::parse(include_str!("input_example.txt"));
        assert_eq!(super::part_2(&parsed, super::stack_input_example()), "MCD");
    }

    #[test]
    fn part_2() {
        let parsed = super::parse(include_str!("input.txt"));
        assert_eq!(super::part_2(&parsed, super::stack_input()), "DCVTCVPCL");
    }
}
