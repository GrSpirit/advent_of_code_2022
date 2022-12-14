use std::str::{self, FromStr};

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
    #[error("Internal Error")]
    #[allow(unused)]
    Internal,
    #[error("Parse error")]
    ParseError,
}

struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap();
        Ok(Self{ x: x.parse()?, y: y.parse()? })
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(_: std::num::ParseIntError) -> Self {
        Error::ParseError
    }
}

fn transform(points: Vec<Vec<Point>>, min_x: i32, max_x: i32, max_y: i32) -> Vec<Vec<u8>> {
    use std::mem::swap;
    let n = (max_y + 1) as usize;
    let m = (max_x - min_x + 1) as usize;
    let mut grid = vec![vec![b'.'; m]; n];
    for row in points {
        for k in 1..row.len() {
            let mut start_j = (row[k - 1].x - min_x) as usize;
            let mut start_i = row[k - 1].y as usize;
            let mut end_j = (row[k].x - min_x) as usize;
            let mut end_i = row[k].y as usize;
            if start_j > end_j {
                swap(&mut start_j, &mut end_j);
            }
            if start_i > end_i {
                swap(&mut start_i, &mut end_i);
            }
            if start_i == end_i {
                for j in start_j..=end_j {
                    grid[start_i][j] = b'#';
                }
            } else {
                for i in start_i..=end_i {
                    grid[i][start_j] = b'#';
                }
            }
        }
    }
    grid
}

fn parse_grid<S: AsRef<str>>(lines: &[S]) -> Vec<Vec<Point>> {
    let mut points = Vec::new();
    for line in lines {
        let path: Vec<Point> = line.as_ref().split(" -> ").map(|p| p.parse::<Point>().unwrap()).collect();
        points.push(path);
    }
    points
}

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<u32, Error> {
    return simulate(lines, |points| {
        let min_x = points.iter().map(|v| v.iter().map(|p| p.x).min().unwrap()).min().unwrap();
        let max_x = points.iter().map(|v| v.iter().map(|p| p.x).max().unwrap()).max().unwrap();
        let max_y = points.iter().map(|v| v.iter().map(|p| p.y).max().unwrap()).max().unwrap();
        (min_x, max_x, max_y)
    });
}

pub fn task2<S: AsRef<str>>(lines: &[S]) -> Result<u32, Error> {
    simulate(lines, |points| {
        let max_y = points.iter().map(|v| v.iter().map(|p| p.y).max().unwrap()).max().unwrap();
        let min_x = 498 - max_y;
        let max_x = 502 + max_y;
        points.push(vec![Point{x: min_x, y: max_y + 2}, Point{ x: max_x, y: max_y + 2}]);
        (min_x, max_x, max_y + 2)
    })
}

fn simulate<S: AsRef<str>, F: FnMut(&mut Vec<Vec<Point>>) -> (i32, i32, i32)>(lines: &[S], mut mutate: F) -> Result<u32, Error> {
    let mut points = parse_grid(lines);
    let (min_x, max_x, max_y) = mutate(&mut points);
    let mut grid = transform(points, min_x, max_x, max_y);
    let mut total = 0;
    loop {
        let n = drop(&mut grid, min_x as usize);
        if n == 0 {
            break;
        }
        total += n;
    }
    Ok(total)
}

fn drop(grid: &mut Vec<Vec<u8>>, center: usize) -> u32 {
    const WIDTH: usize = 500;
    let mut j = WIDTH - center;
    let n = grid.len();
    let m = grid[0].len();
    for i in 1..n {
        if grid[0][WIDTH - center] == b'o' {
            break;
        } else if grid[i][j] == b'.' {
            continue;
        } else {
            if j == 0 {
                grid[i - 1][j] = b'o';
                return (n - i - 1) as u32;
            } else if grid[i][j - 1] == b'.' {
                j -= 1;
            } else if j + 1 == m {
                grid[i - 1][j] = b'o';
                return (n - i - 1) as u32;
            } else if grid[i][j + 1] == b'.' {
                j += 1;
            } else {
                grid[i - 1][j] = b'o';
                return 1;
            }
        }
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &'static str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test1() {
        let lines = DATA.split('\n').collect::<Vec<_>>();
        assert_eq!(Ok(24), task1(&lines));
    }

    #[test]
    fn test2() {
        let lines = DATA.split('\n').collect::<Vec<_>>();
        assert_eq!(Ok(93), task2(&lines));
    }
}
