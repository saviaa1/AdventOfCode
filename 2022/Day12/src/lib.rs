use pathfinding::prelude::{bfs, Matrix};

type Point2d = (usize, usize);
type ParsedTOut = (Matrix<u8>, Point2d, Point2d);
type ParsedTIn = (Matrix<u8>, Point2d, Point2d);
type ReturnT = i64;

pub fn parse(input: &str) -> ParsedTOut {
    let mut matrix = Matrix::from_rows(input.trim().lines().map(|f| f.bytes())).unwrap();
    let start = matrix.keys().find(|&a| matrix[a] == b'S').unwrap();
    let end = matrix.keys().find(|&a| matrix[a] == b'E').unwrap();

    matrix[start] = b'a';
    matrix[end] = b'z';
    
    (matrix, start, end)
}

pub fn part_1(val: &ParsedTIn) -> Result<ReturnT, &'static str> {
    let (ref matrix, start, end) = val;

    let search_result = bfs(
        start,
        |&point| matrix.neighbours(point, false).filter(move |&filter| matrix[filter] <= matrix[point] + 1),
        |point| point == end
    );

    match search_result {
        Some(v) => Ok((v.len() - 1) as i64),
        None => Err("Bfs failed to found results."),
    }
}

pub fn part_2(val: &ParsedTIn) -> Result<ReturnT, &'static str> {
    let (ref matrix, _, end) = val;
    let search_result = bfs(
        end,
        |&point| matrix.neighbours(point, false).filter(move |&filter| 1 + matrix[filter] >= matrix[point]),
        |&point| matrix[point] == b'a',
    );

    match search_result {
        Some(v) => Ok((v.len() - 1) as i64),
        None => Err("Bfs failed to found results."),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1_example() {
        let parsed = super::parse(include_str!("input_example.txt"));
        assert_eq!(super::part_1(&parsed), Ok(31));
    }

    #[test]
    fn part_1() {
        let parsed = super::parse(include_str!("input.txt"));
        assert_eq!(super::part_1(&parsed), Ok(370));
    }

    #[test]
    fn part_2_example() {
        let parsed = super::parse(include_str!("input_example.txt"));
        assert_eq!(super::part_2(&parsed), Ok(29));
    }

    #[test]
    fn part_2() {
        let parsed = super::parse(include_str!("input.txt"));
        assert_eq!(super::part_2(&parsed), Ok(363));
    }
}
