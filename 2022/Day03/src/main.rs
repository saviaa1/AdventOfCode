use std::collections::HashSet;

static ASCII: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn parse(input: &str) -> Vec<(HashSet<char>, HashSet<char>)> {
    input
        .trim()
        .lines()
        .map(|c| c.split_at(c.len()/2))
        .map(|c| (c.0.chars().collect(), c.1.chars().collect()))
        .collect()
}

pub fn part_1(val: &[(HashSet<char>, HashSet<char>)]) -> i64 {
    let mut count: i64 = 0;
    for m in val {

        let inter: HashSet<_> = m.0.intersection(&m.1).collect();

        assert!(inter.len() == 1);

        let c = match inter.iter().next() {
            Some(&x) => *x,
            None => panic!("No type"),
        };

        let pos = ASCII.chars().position(|x| x == c).unwrap() as i64;
        count += pos + 1;
    }
    count
}

pub fn part_2(val: &[(HashSet<char>, HashSet<char>)]) -> i64 {
    let mut count: i64 = 0;
    assert!(val.len() % 3 == 0);

    let mut i = 0;
    while i < val.len() {
        let mut a: HashSet<char> = HashSet::new();
        let mut b: HashSet<char> = HashSet::new();
        let mut c: HashSet<char> = HashSet::new();
        a.extend(val[i+0].0.clone());
        a.extend(val[i+0].1.clone());
        b.extend(val[i+1].0.clone());
        b.extend(val[i+1].1.clone());
        c.extend(val[i+2].0.clone());
        c.extend(val[i+2].1.clone());

        let d: HashSet<char> = a.into_iter().filter(|e| b.contains(e)).collect();
        let e: HashSet<char> = d.into_iter().filter(|e| c.contains(e)).collect();
        
        assert!(e.len() == 1);

        let pos = ASCII.chars().position(|x| x == *e.iter().next().unwrap()).unwrap() as i64;
        count += pos + 1;

        i+=3;
    }

    count
}

pub fn main() {
    let parsed = parse(include_str!("input.txt"));
    println!("2022 Day02");
    println!("Part 1: {}", part_1(&parsed));
    println!("Part 2: {}", part_2(&parsed));
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1_example() {
        let parsed = super::parse(include_str!("input_example.txt"));
        assert_eq!(super::part_1(&parsed), 157);
    }

    #[test]
    fn part_1() {
        let parsed = super::parse(include_str!("input.txt"));
        assert_eq!(super::part_1(&parsed), 7903);
    }

    #[test]
    fn part_2_example() {
        let parsed = super::parse(include_str!("input_example.txt"));
        assert_eq!(super::part_2(&parsed), 70);
    }

    #[test]
    fn part_2() {
        let parsed = super::parse(include_str!("input.txt"));
        assert_eq!(super::part_2(&parsed), 2548);
    }
}
