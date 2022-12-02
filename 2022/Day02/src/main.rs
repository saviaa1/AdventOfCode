fn parse(input: &str) -> Vec<(char, char)> {
    input
        .trim()
        .lines()
        .map(|c| (c.chars().next().unwrap(), c.chars().nth(2).unwrap()))
        .collect()
}

enum Result {
    Win,
    Lose,
    Draw,
}

enum Weapon {
    Rock,
    Paper,
    Scissor,
}

fn get_weapon(c: char) -> Weapon {
    match c {
        'A' | 'X' => Weapon::Rock,
        'B' | 'Y' => Weapon::Paper,
        'C' | 'Z' => Weapon::Scissor,
        _         => unreachable!(),
    }
}

fn get_result(c: char) -> Result {
    match c {
        'X' => Result::Lose,
        'Y' => Result::Draw,
        'Z' => Result::Win,
        _ => unreachable!(),
    }
}

fn match_end_result(opponen: &Weapon, own: &Weapon) -> Result {
    match (opponen, own) {
        (Weapon::Rock, Weapon::Paper) => Result::Win,
        (Weapon::Scissor, Weapon::Rock) => Result::Win,
        (Weapon::Paper, Weapon::Scissor) => Result::Win,
        (Weapon::Rock, Weapon::Scissor) => Result::Lose,
        (Weapon::Scissor, Weapon::Paper) => Result::Lose,
        (Weapon::Paper, Weapon::Rock) => Result::Lose,
        _ => Result::Draw,
    }
}

fn match_weapon_result(opponen: &Weapon, result: &Result) -> Weapon {
    match (opponen, result) {
        (Weapon::Rock, Result::Draw) => Weapon::Rock,
        (Weapon::Scissor, Result::Win) => Weapon::Rock,
        (Weapon::Paper, Result::Lose) => Weapon::Rock,
        (Weapon::Rock, Result::Win) => Weapon::Paper,
        (Weapon::Scissor, Result::Lose) => Weapon::Paper,
        (Weapon::Paper, Result::Draw) => Weapon::Paper,
        _ => Weapon::Scissor,
    }
}

fn result_point(res: &Result) -> i64 {
    match res {
        Result::Win => 6,
        Result::Draw => 3,
        Result::Lose => 0,
    }
}

fn weapon_point(weapon: &Weapon) -> i64 {
    match weapon {
        Weapon::Rock => 1,
        Weapon::Paper => 2,
        Weapon::Scissor => 3,
    }
}

pub fn part_a(val: &Vec<(char, char)>) -> i64 {
    let mut score: i64 = 0;
    for m in val {
        //Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock.
        //A for Rock, B for Paper, and C for Scissors
        //X for Rock, Y for Paper, and Z for Scissors
        //1 for Rock, 2 for Paper, and 3 for Scissors
        //0 if you lost, 3 if the round was a draw, and 6 if you won
        let oponent = get_weapon(m.0);
        let own = get_weapon(m.1);
        let res = match_end_result(&oponent, &own);
        score += result_point(&res) + weapon_point(&own);
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
        let oponent = get_weapon(m.0);
        let res = get_result(m.1);
        let own = match_weapon_result(&oponent, &res);
        score += result_point(&res) + weapon_point(&own); 
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
