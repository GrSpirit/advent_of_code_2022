use std::collections::VecDeque;

#[derive(Debug, Clone, thiserror::Error, PartialEq)]
pub enum Error {
    #[error("Start position not found")]
    StartNotFound,
    #[error("Target not found")]
    TargetNotFound,
}

fn to_grid<S: AsRef<str>>(lines: &[S]) -> Vec<Vec<u8>> {
    lines.iter().map(|l| l.as_ref().bytes().collect()).collect()
}

fn start_position(grid: &Vec<Vec<u8>>) -> Option<(usize, usize)> {
    for (i, row) in grid.iter().enumerate() {
        if let Some(j) = row.iter().position(|x| *x == b'E') {
            return Some((i, j));
        }
    }
    None
}

fn can_visit(next_val: u8, prev_val: u8) -> bool {
    (next_val.is_ascii_lowercase() && prev_val.is_ascii_lowercase() && next_val >= prev_val - 1) ||
    (prev_val == b'E' && next_val == b'z') ||
    (next_val == b'S' && (prev_val == b'a' || prev_val == b'b'))
}

pub fn find_target<F: Fn(u8)->bool>(grid: Vec<Vec<u8>>, is_finish: F) -> Result<i32, Error> {
    const DIRECTIONS: &[(i32, i32)] = &[(-1, 0), (0, -1), (1, 0), (0, 1)];
    let n = grid.len();
    let m = grid[0].len();
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let (start_row, start_col) = start_position(&grid).ok_or(Error::StartNotFound)?;
    let mut queue = VecDeque::new();
    queue.push_back((start_row, start_col, 0));
    visited[start_row][start_col] = true;
    while let Some((row, col, length)) = queue.pop_front() {
        if is_finish(grid[row][col]) {
            return Ok(length);
        }
        for (i, j) in DIRECTIONS {
            let (next_row, next_col) = ((row as i32 + i) as usize, (col as i32 + j) as usize);
            if next_row < n && next_col < m && !visited[next_row][next_col] && can_visit(grid[next_row][next_col], grid[row][col]) {
                visited[next_row][next_col] = true;
                queue.push_back((next_row, next_col, length + 1));
            }
        }
    }
    Err(Error::TargetNotFound)
}

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<i32, Error> {
    find_target(to_grid(lines), |x| x == b'S')
}

pub fn task2<S: AsRef<str>>(lines: &[S]) -> Result<i32, Error> {
    find_target(to_grid(lines), |x| x == b'S' || x == b'a')
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &'static str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test1() {
        let lines = DATA.split('\n').collect::<Vec<_>>();
        assert_eq!(Ok(31), task1(&lines));
    }

    #[test]
    fn test2() {
        let lines = DATA.split('\n').collect::<Vec<_>>();
        assert_eq!(Ok(29), task2(&lines));
    }
}
