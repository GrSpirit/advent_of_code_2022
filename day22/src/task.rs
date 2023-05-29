use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
    #[error("Internal error")]
    #[allow(unused)]
    Internal,
    #[error("Parse field error")]
    ParseField(u8),
    #[error("Parse grid error")]
    ParseGrid,
    #[error("Parse path error")]
    ParsePath,
    #[error("Wrong grid format")]
    WrongGridFormat
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Field {
    Empty,
    Space,
    Wall,
}

impl TryFrom<u8> for Field {
    type Error = Error;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            b' ' => Ok(Field::Empty),
            b'.' => Ok(Field::Space),
            b'#' => Ok(Field::Wall),
            v => Err(Error::ParseField(v))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Path {
    Forward(usize),
    Left,
    Right,
}

impl FromStr for Path {
    type Err = Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "L" => Ok(Path::Left),
            "R" => Ok(Path::Right),
            ns => ns.parse().map(|n| Path::Forward(n)).map_err(|_| Error::ParsePath)
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl From<Direction> for i32 {
    fn from(d: Direction) -> Self {
        use Direction::*;
        match d {
            Right => 0,
            Down => 1,
            Left => 2,
            Up => 3,
        }
    }
}

fn parse_path(path: &str) -> Result<Vec<Path>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)|(L)|(R)").unwrap();
    }
    let result: Option<Vec<Path>> = RE.find_iter(path).map(|part| part.as_str().parse().ok()).collect();
    result.ok_or(Error::ParsePath)
}

fn parse_input<S: AsRef<str>>(lines: &[S]) -> Result<(Vec<Vec<Field>>, Vec<Path>)> {
    let grid_height = lines.iter().position(|s| s.as_ref().is_empty()).ok_or(Error::WrongGridFormat)?;
    let max_width = lines.iter().take(grid_height).map(|s| s.as_ref().len()).max().unwrap();
    
    let grid: Option<Vec<Vec<Field>>> = lines.iter().take(grid_height).map(|row| {
        row.as_ref()
        .bytes()
        .map(|b| b.try_into().ok())
        .chain(std::iter::repeat(Some(Field::Empty)))
        .take(max_width)
        .collect()
    }).collect();
    Ok((grid.ok_or(Error::ParseGrid)?, parse_path(lines.last().unwrap().as_ref())?))
}

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<i32> {
    fn move_to(grid: &Vec<Vec<Field>>, position: (usize, usize), dir: Direction) -> Option<(usize, usize)> {
        use crate::wrapper::*;
        let n = grid.len();
        let m = grid[0].len();
        let (mut wi, mut wj) = (Wrapper(position.0), Wrapper(position.1));
        loop {
            match dir {
                Direction::Right => {
                    wj.inc(m);
                }
                Direction::Left => {
                    wj.dec(m);
                },
                Direction::Up => {
                    wi.dec(n);
                },
                Direction::Down => {
                    wi.inc(n);
                }
            }
            let (i, j) = (wi.0, wj.0);
            match grid[i][j] {
                Field::Wall => return None,
                Field::Space => return Some((i, j)),
                _ => {} // continue
            }
        }
    }
    let (grid, path) = parse_input(lines)?;
    let mut dir = Direction::Right;
    let mut position = (0usize, grid[0].iter().position(|f| *f == Field::Space).unwrap());
    for p in path {
        match p {
            Path::Forward(n) => {
                for _ in 0..n {
                    if let Some(next_position) = move_to(&grid, position, dir) {
                        position = next_position;
                    } else {
                        break;
                    }
                }
            },
            Path::Left => {
                dir = match dir {
                    Direction::Right => Direction::Up,
                    Direction::Up => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Down => Direction::Right,
                };
            },
            Path::Right => {
                dir = match dir {
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Up => Direction::Right,
                };
            }
        }
    }
    let res = (position.0 as i32 + 1) * 1000 + (position.1 as i32 + 1) * 4 + i32::from(dir);
    Ok(res)
}

#[allow(dead_code)]
pub fn task2<S: AsRef<str>>(_lines: &[S]) -> Result<i32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str =
"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    fn test1() {
        let lines = DATA.split('\n').collect::<Vec<_>>();
        assert_eq!(Ok(6032), task1(&lines));
    }

    #[test]
    fn test2() {
        let lines = DATA.split('\n').map(|s| s.trim()).collect::<Vec<_>>();
        assert_eq!(Ok(0), task2(&lines));
    }

    #[test]
    fn test_parse_path() {
        use Path::*;
        let path = "10R5LR10L4RL5";
        let result = parse_path(path);
        assert_eq![Ok(vec![Forward(10), Right, Forward(5), Left, Right, Forward(10), Left, Forward(4), Right, Left, Forward(5)]), result]
    }
}
