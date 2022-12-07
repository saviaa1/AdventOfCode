use std::{collections::HashMap, iter, fmt, str::FromStr};

type ParsedTOut<> = Vec<Line>;
type ParsedTIn<> = Vec<Line>;
type ReturnT = i64;

#[derive(Debug, Clone)]
pub enum Command {
    Cd(String),
    Ls,
}

#[derive(Debug, Clone)]
pub enum Line {
    Command(Command),
    File(String, usize),
    Folder(String),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseLineError;

impl fmt::Display for ParseLineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parsing string to Line struct encountered error")
    }
}

impl FromStr for Line {
    type Err = ParseLineError;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut line = line.split(' ');
        let first = line.next().unwrap();
        let second = line.next().unwrap();
        match first {
            "$" => match second {
                "cd" => Ok(Line::Command(Command::Cd(line.next().unwrap().to_string()))),
                "ls" => Ok(Line::Command(Command::Ls)),
                _ => Err(ParseLineError),
            },
            "dir" => Ok(Line::Folder(second.to_string())),
            _ => Ok(Line::File(second.to_string(), first.parse().unwrap())),
        }
    }
}

pub fn parse(input: &str) -> ParsedTOut {
    input
        .trim()
        .lines()
        .map(|line| Line::from_str(line).unwrap())
        .collect()
}

pub fn part_1(val: &ParsedTIn) -> ReturnT {
    let mut directory: Vec<String> = Vec::new();
    let mut folders: HashMap<String, usize> = HashMap::new();

    for line in val {
        match line {
            Line::Command(command) => match command {
                Command::Cd(path) => match path.as_str() {  //& cd XXX
                    "/" => {
                        directory.clear();
                        directory.push(path.clone())
                    }
                    ".." => {directory.pop();}
                    _ => directory.push(path.clone()),
                },
                Command::Ls => (),                                  //& ls
            },
            Line::File(name, size) => {            //size fileName
                let mut directory = directory.iter().chain(iter::once(name));
                let mut path = directory.next().unwrap().clone();
                for dir in directory {
                    folders.entry(path.clone()).and_modify(|f| { *f += size }).or_insert(*size);
                    path += dir;
                    path += "/";
                }
            }
            Line::Folder(_) => (),                                  //dir XXX
        }
    }

    folders.values().filter(|&&size| size < 100000).sum::<usize>() as i64
}

pub fn part_2(val: &ParsedTIn) -> ReturnT {
    let mut directory: Vec<String> = Vec::new();
    let mut folders = HashMap::new();

    for line in val {
        match line {
            Line::Command(command) => match command {
                Command::Cd(path) => match path.as_str() {
                    "/" => {
                        directory.clear();
                        directory.push(path.to_string())
                    }
                    ".." => {
                        directory.pop().unwrap();
                    }
                    _ => directory.push(path.to_string()),
                },
                Command::Ls => (),
            },
            Line::File(name, size) => {
                let mut directory = directory.iter().chain(iter::once(name));
                let mut path = directory.next().unwrap().clone();
                for dir in directory {
                    folders.entry(path.clone()).and_modify(|f| { *f += size }).or_insert(*size);
                    path += dir;
                    path += "/";
                }
            }
            Line::Folder(_) => (),
        }
    }
    
    let missing_space = 30_000_000 - (70_000_000 - folders.get("/").unwrap());
    let size: &usize = folders
        .values()
        .filter(|&&size| size >= missing_space)
        .min()
        .unwrap();

    (*size) as i64
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1_example() {
        let parsed = super::parse(include_str!("input_example.txt"));
        assert_eq!(super::part_1(&parsed), 95437);
    }

    #[test]
    fn part_1() {
        let parsed = super::parse(include_str!("input.txt"));
        assert_eq!(super::part_1(&parsed), 1350966);
    }

    #[test]
    fn part_2_example() {
        let parsed = super::parse(include_str!("input_example.txt"));
        assert_eq!(super::part_2(&parsed), 24933642);
    }

    #[test]
    fn part_2() {
        let parsed = super::parse(include_str!("input.txt"));
        assert_eq!(super::part_2(&parsed), 6296435);
    }
}
