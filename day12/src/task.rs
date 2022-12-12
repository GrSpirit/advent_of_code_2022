use std::error::Error;
use std::collections::VecDeque;

pub type BoxResult<T> = Result<T, Box<dyn Error>>;

fn to_grid<S: AsRef<str>>(lines: &[S]) -> Vec<Vec<u8>> {
    lines.iter().map(|l| l.as_ref().bytes().collect()).collect()
}

fn start_position(grid: &Vec<Vec<u8>>) -> (usize, usize) {
    for (i, row) in grid.iter().enumerate() {
        if let Some(j) = row.iter().position(|x| *x == b'S') {
            return (i, j);
        }
    }
    (0, 0)
}

fn can_visit(next_val: u8, prev_val: u8) -> bool {
    (next_val >= b'a' && next_val <= prev_val + 1) ||
    (prev_val == b'z' && next_val == b'E') || (prev_val == b'S' && (next_val == b'a' || next_val == b'b'))
}

pub fn task1<S: AsRef<str>>(lines: &[S]) -> BoxResult<i32> {
    let grid = to_grid(lines);
    let n = grid.len();
    let m = grid[0].len();
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    //let mut path = Vec::new();
    let (start_row, start_col) = start_position(&grid);
    //dfs(&grid, &mut path, &mut visited, start_row, start_col);
    let mut queue = VecDeque::new();
    queue.push_back((start_row, start_col, 0));
    let directions = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    visited[start_row][start_col] = true;
    while let Some((row, col, length)) = queue.pop_front() {
        if grid[row][col] == b'E' {
            return Ok(length);
        }
        for (i, j) in &directions {
            let (next_row, next_col) = ((row as i32 + i) as usize, (col as i32 + j) as usize);
            if next_row < n && next_col < m && !visited[next_row][next_col] && can_visit(grid[next_row][next_col], grid[row][col]) {
                visited[next_row][next_col] = true;
                queue.push_back((next_row, next_col, length + 1));
            }
        }
    }
    Err(Box::<dyn Error>::from("not found"))
}

pub fn task2<S: AsRef<str>>(lines: &[S]) -> BoxResult<u32> {
    Ok(lines.len() as u32)
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
        assert_eq!(31, task1(&lines).unwrap());
    }

    #[test]
    fn test2() {
        let lines = DATA.split('\n').collect::<Vec<_>>();
        assert_eq!(5, task2(&lines).unwrap());
    }
}
