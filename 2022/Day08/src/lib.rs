type ParsedTOut = Vec<Vec<i8>>;
type ParsedTIn = Vec<Vec<i8>>;
type ReturnT = i64;

pub fn parse(input: &str) -> ParsedTOut {
    input
        .trim()
        .lines()
        .map(|f|{ f
            .trim()
            .chars()
            .map(|g| g.to_digit(10).unwrap() as i8)
            .collect()
        })
        .collect::<ParsedTOut>()
}

pub fn part_1(val: &ParsedTIn) -> ReturnT {
    let width = val[0].len();
    let mut count: i64 = 0;
    
    let mut masked = val
        .iter()
        .map(|f| f.iter().map(|g| (*g as i8, false)).collect::<Vec<(i8, bool)>>())
        .collect::<Vec<Vec<(i8, bool)>>>();

    //left, right
    for i in masked.iter_mut() {
        let mut highest: i8 = -1;
        for j in i.iter_mut() {
            if highest < j.0 {
                highest = j.0;
                if !j.1 {
                    j.1 = true;
                    count += 1;
                }
            }
        }
        highest = -1;
        for j in i.iter_mut().rev() {
            if highest < j.0 {
                highest = j.0;
                if !j.1 {
                    j.1 = true;
                    count += 1;
                }
            }
        }
    }
    
    //up, down
    for j in 0..width {
        let mut highest: i8 = -1;
        for i in masked.iter_mut() {
            if highest < i[j].0 {
                highest = i[j].0;
                if !i[j].1 {
                    i[j].1 = true;
                    count += 1;
                }
            }
        }
        highest = -1;
        for i in masked.iter_mut().rev() {
            if highest < i[j].0 {
                highest = i[j].0;
                if !i[j].1 {
                    i[j].1 = true;
                    count += 1;
                }
            }
        }
    }
    count
}

pub fn part_2(val: &ParsedTIn) -> ReturnT {
    let width = val[0].len();
    let height = val.len();

    let mut tree_rank: Vec<Vec<i64>> = vec![vec![0; width]; height];

    for y in 0..height {
        for x in 0..width {
            let cur = val[y][x];
            let mut up: u8 = 0;
            let mut dow: u8 = 0;
            let mut le: u8 = 0;
            let mut ri: u8 = 0;
            for u in (0..y).rev() {
                if val[u][x] < cur {
                    up += 1;
                } else if val[u][x] >= cur {
                    up += 1;
                    break;
                } else {
                    break;
                }
            }
            for d in val.iter().take(height).skip(y+1) {
                if d[x] < cur {
                    dow += 1;
                } else if d[x] >= cur {
                    dow += 1;
                    break;
                } else {
                    break;
                }
            }
            for l in (0..x).rev() {
                if val[y][l] < cur {
                    le += 1;
                } else if val[y][l] >= cur {
                    le += 1;
                    break;
                } else {
                    break;
                }
            }
            for r in (x+1)..width {
                if val[y][r] < cur {
                    ri += 1;
                } else if val[y][r] >= cur {
                    ri += 1;
                    break;
                } else {
                    break;
                }
            }
            tree_rank[y][x] = (up as i64)*(dow as i64)*(le as i64)*(ri as i64);
        }
    }
    *tree_rank.iter().map(|f| f.iter().max().unwrap()).max().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1_example() {
        let parsed = super::parse(include_str!("input_example.txt"));
        assert_eq!(super::part_1(&parsed), 21);
    }

    #[test]
    fn part_1() {
        let parsed = super::parse(include_str!("input.txt"));
        assert_eq!(super::part_1(&parsed), 1669);
    }

    #[test]
    fn part_2_example() {
        let parsed = super::parse(include_str!("input_example.txt"));
        assert_eq!(super::part_2(&parsed), 8);
    }

    #[test]
    fn part_2() {
        let parsed = super::parse(include_str!("input.txt"));
        assert_eq!(super::part_2(&parsed), 331344);
    }
}
