type ParsedTOut = Vec<(char, char)>;
type ParsedTIn = [(char, char)];
type ReturnT = i64;

pub fn parse(input: &str) -> ParsedTOut {
    input
        .trim()
        .lines()
        .map(|c| (c.chars().next().unwrap(), c.chars().nth(2).unwrap()))
        .collect()
}

#[derive(Debug)]
enum Result {
    Win,
    Lose,
    Draw,
}

#[derive(Debug)]
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

pub fn part_1(val: &ParsedTIn) -> ReturnT {
    val
        .iter()
        .map(|m| result_point(&match_end_result(&get_weapon(m.0), &get_weapon(m.1))) + weapon_point(&get_weapon(m.1)))
        .sum()
}

pub fn part_2(val: &ParsedTIn) -> ReturnT {
    let mut score: i64 = 0;
    for m in val {
        let oponent = get_weapon(m.0);
        let res = get_result(m.1);
        let own = match_weapon_result(&oponent, &res);
        score += result_point(&res) + weapon_point(&own); 
    }
    score
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1_example() {
        let parsed = super::parse(include_str!("input_example.txt"));
        assert_eq!(super::part_1(&parsed), 15);
    }

    #[test]
    fn part_1() {
        let parsed = super::parse(include_str!("input.txt"));
        assert_eq!(super::part_1(&parsed), 12794);
    }

    #[test]
    fn part_2_example() {
        let parsed = super::parse(include_str!("input_example.txt"));
        assert_eq!(super::part_2(&parsed), 12);
    }

    #[test]
    fn part_2() {
        let parsed = super::parse(include_str!("input.txt"));
        assert_eq!(super::part_2(&parsed), 14979);
    }
}
