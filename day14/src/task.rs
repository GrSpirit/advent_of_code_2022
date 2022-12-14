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

fn transform1(points: Vec<Vec<Point>>, min_x: i32, _min_y: i32, max_x: i32, max_y: i32) -> Vec<Vec<u8>> {
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
                std::mem::swap(&mut start_j, &mut end_j);
            }
            if start_i > end_i {
                std::mem::swap(&mut start_i, &mut end_i);
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

fn parse_grid1<S: AsRef<str>>(lines: &[S]) -> (Vec<Vec<u8>>, usize) {
    let (mut min_x, mut max_x) = (i32::MAX, i32::MIN);
    let (mut min_y, mut max_y) = (i32::MAX, i32::MIN);
    let mut points = Vec::new();
    for line in lines {
        let path: Vec<Point> = line.as_ref().split(" -> ").map(|p| p.parse::<Point>().unwrap()).collect();
        min_x = path.iter().map(|p| p.x).min().unwrap().min(min_x);
        max_x = path.iter().map(|p| p.x).max().unwrap().max(max_x);
        min_y = path.iter().map(|p| p.y).min().unwrap().min(min_y);
        max_y = path.iter().map(|p| p.y).max().unwrap().max(max_y);
        points.push(path);
    }
    (transform1(points, min_x, min_y, max_x, max_y), min_x as usize)
}

fn drop1(grid: &mut Vec<Vec<u8>>, shift: usize) -> bool {
    let mut j = 500 - shift;
    let n = grid.len();
    let m = grid[0].len();
    for i in 1..n {
        if grid[i][j] == b'.' {
            continue;
        } else {
            if j == 0 {
                return false;
            } else if grid[i][j - 1] == b'.' {
                j -= 1;
            } else if j + 1 == m {
                return false;
            } else if grid[i][j + 1] == b'.' {
                j += 1;
            } else {
                grid[i - 1][j] = b'o';
                return true;
            }
        }
    }
    return false;
}

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<u32, Error> {
    let (mut grid, shift) = parse_grid1(lines);
    let mut total = 0;
    while drop1(&mut grid, shift) {
        total += 1;
        for row in &grid {
            println!("{}", str::from_utf8(row).unwrap());
        }
    }
    Ok(total)
}

fn transform2(points: Vec<Vec<Point>>, min_x: i32, _min_y: i32, max_x: i32, max_y: i32) -> Vec<Vec<u8>> {
    let n = (max_y + 3) as usize;
    let m = (max_x - min_x + 1) as usize;
    let mut grid = vec![vec![b'.'; m]; n];
    for row in points {
        for k in 1..row.len() {
            let mut start_j = (row[k - 1].x - min_x) as usize;
            let mut start_i = row[k - 1].y as usize;
            let mut end_j = (row[k].x - min_x) as usize;
            let mut end_i = row[k].y as usize;
            if start_j > end_j {
                std::mem::swap(&mut start_j, &mut end_j);
            }
            if start_i > end_i {
                std::mem::swap(&mut start_i, &mut end_i);
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
    grid[n - 1] = vec![b'#'; m];
    grid
}

fn parse_grid2<S: AsRef<str>>(lines: &[S]) -> (Vec<Vec<u8>>, usize) {
    let (mut min_x, mut max_x) = (i32::MAX, i32::MIN);
    let (mut min_y, mut max_y) = (i32::MAX, i32::MIN);
    let mut points = Vec::new();
    for line in lines {
        let path: Vec<Point> = line.as_ref().split(" -> ").map(|p| p.parse::<Point>().unwrap()).collect();
        min_x = path.iter().map(|p| p.x).min().unwrap().min(min_x);
        max_x = path.iter().map(|p| p.x).max().unwrap().max(max_x);
        min_y = path.iter().map(|p| p.y).min().unwrap().min(min_y);
        max_y = path.iter().map(|p| p.y).max().unwrap().max(max_y);
        points.push(path);
    }
    (transform2(points, 498 - max_y, min_y, 502 + max_y, max_y), 498 - max_y as usize)
}

fn drop2(grid: &mut Vec<Vec<u8>>, shift: usize) -> u32 {
    let mut j = 500 - shift;
    let n = grid.len();
    let m = grid[0].len();
    for i in 1..n {
        if grid[0][500 - shift] == b'o' {
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

pub fn task2<S: AsRef<str>>(lines: &[S]) -> Result<u32, Error> {
    let (mut grid, shift) = parse_grid2(lines);
    let mut total = 0;
    loop {
        let n = drop2(&mut grid, shift);
        if n == 0 {
            break;
        }
        total += n;
    }
    Ok(total)
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
