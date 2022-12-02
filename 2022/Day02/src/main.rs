fn parse(input: &str) -> Vec<(char, char)> {
    let mut vec: Vec<(char, char)> = Vec::new();
    for line in input.split("\r\n") {
        let line = line.trim();
        if !line.is_empty() {
            let tu: (char, char) = (line.chars().next().unwrap(), line.chars().nth(2).unwrap());
            vec.push(tu);
        }
    }
    vec
}

pub fn part_a(val: &Vec<(char, char)>) -> i64 {
    let mut score: i64 = 0;
    for m in val {
        //Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock.
        //A for Rock, B for Paper, and C for Scissors
        //X for Rock, Y for Paper, and Z for Scissors
        //1 for Rock, 2 for Paper, and 3 for Scissors
        //0 if you lost, 3 if the round was a draw, and 6 if you won
        if m.0 == 'A' && m.1 == 'Z' { //Rock defeats Scissors, lost
            score += 3+0;
        } else if m.0 == 'C' && m.1 == 'Y' { //Scissors defeats Paper, lost
            score += 2+0;
        } else if m.0 == 'B' && m.1 == 'X' { //Paper defeats Rock, lost
            score += 1+0;
        } else if m.0 == 'C' && m.1 == 'X' { //Rock defeats Scissors, won
            score += 1+6;
        } else if m.0 == 'B' && m.1 == 'Z' { //Scissors defeats Paper, won
            score += 3+6;
        } else if m.0 == 'A' && m.1 == 'Y' { //Paper defeats Rock, won
            score += 2+6;
        } else { //draw
            if m.1 == 'X' {
                score += 1+3;
            } else if m.1 == 'Y' {
                score += 2+3;
            } else {
                score += 3+3;
            }
        }
    }
    score
}

pub fn part_b(val: &Vec<(char, char)>) -> i64 {
    let mut score: i64 = 0;
    for m in val {
        //Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock.
        //A Rock, B Paper, C Scissors
        //X lose, Y draw , Z win
        //1 for Rock, 2 for Paper, and 3 for Scissors
        //0 if you lost, 3 if the round was a draw, and 6 if you won
        if m.0 == 'A' && m.1 == 'X' { //Rock lose, Scissors
            score += 3+0;
        } else if m.0 == 'A' && m.1 == 'Y' { //Rock draw, Rock
            score += 1+3;
        } else if m.0 == 'A' && m.1 == 'Z' { //Rock win, Paper
            score += 2+6;
        } else if m.0 == 'B' && m.1 == 'X' { //Paper lost, Rock
            score += 1+0;
        } else if m.0 == 'B' && m.1 == 'Y' { //Paper draw, Paper
            score += 2+3;
        } else if m.0 == 'B' && m.1 == 'Z' { //Paper win, Scissor
            score += 3+6;
        } else if m.0 == 'C' && m.1 == 'X' { //Scissors lost, Paper
            score += 2+0;
        } else if m.0 == 'C' && m.1 == 'Y' { //Scissors draw, Scissors
            score += 3+3;
        } else if m.0 == 'C' && m.1 == 'Z' { //Scissors win, Rock
            score += 1+6;
        } else {
            panic!("Should not end here");
        }
    }
    score
}

pub fn main() {
    let parsed = parse(include_str!("input.txt"));
    println!("2022 Day02");
    println!("Part 1: {}", part_a(&parsed));
    println!("Part 2: {}", part_b(&parsed));
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a_example() {
        let parsed = super::parse(include_str!("input1.txt"));
        assert_eq!(super::part_a(&parsed), 15);
    }

    #[test]
    fn part_a() {
        let parsed = super::parse(include_str!("input.txt"));
        assert_eq!(super::part_a(&parsed), 12794);
    }

    #[test]
    fn part_b_example() {
        let parsed = super::parse(include_str!("input1.txt"));
        assert_eq!(super::part_b(&parsed), 12);
    }

    #[test]
    fn part_b() {
        let parsed = super::parse(include_str!("input.txt"));
        assert_eq!(super::part_b(&parsed), 14979);
    }
}
