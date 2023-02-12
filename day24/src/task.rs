use bitflags::bitflags;
use std::collections::{VecDeque, HashSet};
use std::fmt::Display;
use std::mem::swap;

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
    #[error("Internal error")]
    #[allow(unused)]
    Internal,
    #[error("EndLoop")]
    EndLoop,
}

bitflags! {
    struct Field: u8 {
        const EMPTY     = 0;
        const LEFT      = 1 << 0;
        const RIGHT     = 1 << 1;
        const UP        = 1 << 2;
        const DOWN      = 1 << 3;
        const WALL      = 1 << 4;
    }
}

impl Field {
    fn clear(&mut self) {
        self.bits = 0;
    }
}

impl TryFrom<u8> for Field {
    type Error = Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'#' => Ok(Field::WALL),
            b'.' => Ok(Field::EMPTY),
            b'<' => Ok(Field::LEFT),
            b'>' => Ok(Field::RIGHT),
            b'v' => Ok(Field::DOWN),
            b'^' => Ok(Field::UP),
            _ => Err(Error::Internal)
        }
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.bits().count_ones() > 1 {
            write!(f, "{}", self.bits().count_ones())
        } else if self.contains(Field::LEFT) {
            write!(f, "<")
        } else if self.contains(Field::RIGHT) {
            write!(f, ">")
        } else if self.contains(Field::UP) {
            write!(f, "^")
        } else if self.contains(Field::DOWN) {
            write!(f, "v")
        } else if self.contains(Field::WALL) {
            write!(f, "#")
        } else {
            write!(f, ".")
        }
    }
}

type Grid = Vec<Vec<Field>>;

fn parse_input<S: AsRef<str>>(lines: &[S]) -> Result<Grid, Error> {
    Ok(lines.iter().map(|row| row.as_ref().bytes().map(|b| Field::try_from(b).unwrap()).collect()).collect())
}

fn update_state(grid: &mut Grid, tmp_grid: &mut Grid) {
    let n = grid.len();
    let m = grid[0].len();
    tmp_grid[1..n-1].iter_mut().for_each(|row| row[1..m-1].iter_mut().for_each(|f| f.clear()));
    for (i, row) in grid.iter().enumerate().skip(1).take(n - 2) {
        for (j, &x) in row.iter().enumerate().skip(1).take(m - 2) {
            if x.contains(Field::UP) {
                let ni = if i == 1 { n - 2 } else { i - 1 };
                tmp_grid[ni][j] |= Field::UP;
            }
            if x.contains(Field::DOWN) {
                let ni = if i == n - 2 { 1 } else { i + 1 };
                tmp_grid[ni][j] |= Field::DOWN;
            }
            if x.contains(Field::LEFT) {
                let nj = if j == 1 { m - 2 } else { j - 1 };
                tmp_grid[i][nj] |= Field::LEFT;
            }
            if x.contains(Field::RIGHT) {
                let nj = if j == m - 2 { 1 } else { j + 1 };
                tmp_grid[i][nj] |= Field::RIGHT;
            }
        }
    }
    swap(grid, tmp_grid)
}

#[allow(unused)]
fn print_grid(grid: &Grid, x: usize, y: usize) {
    for (i, row) in grid.iter().enumerate() {
        for (j, &field) in row.iter().enumerate() {
            if (i, j) == (x, y) {
                if field.bits().count_ones() > 0 {
                    print!("X");
                } else {
                    print!("E");
                }
            } else {
                print!("{}", field);
            }
        }
        println!();
    }
    println!();
}

fn play(grid: &mut Grid, start: (usize, usize), finish: (usize, usize)) -> Result<i32, Error> {
    let mut tmp_grid = grid.clone();
    let mut q = VecDeque::new();
    let mut visited = HashSet::new();
    let (n, m) = (grid.len(), grid[0].len());
    let mut current_step = -1;
    q.push_back((start.0, start.1, 0));
    while let Some((i, j, step)) = q.pop_front() {
        if step != current_step {
            update_state(grid, &mut tmp_grid);
            current_step = step;
        }
        let mut visit = |x, y| {
            visited.insert(((step + 1) % ((n - 2) as i32 * (m - 2) as i32), x, y))
        };
        if grid[i][j].is_empty() && visit(i, j) {
            q.push_back((i, j, step + 1));
        }
        if i > 0 && grid[i - 1][j].is_empty() && visit(i - 1, j) {
            if (i - 1, j) == finish {
                return Ok(step + 1);
            }
            q.push_back((i - 1, j, step + 1));
        }
        if j > 0 && grid[i][j - 1].is_empty() && visit(i, j - 1) {
            if (i, j - 1) == finish {
                return Ok(step + 1);
            }
            q.push_back((i, j - 1, step + 1));
        }
        if i + 1 < n && grid[i + 1][j].is_empty() && visit(i + 1, j) {
            if (i + 1, j) == finish {
                return Ok(step + 1);
            }
            q.push_back((i + 1, j, step + 1));
        }
        if j + 1 < m && grid[i][j + 1].is_empty() && visit(i, j + 1) {
            if (i, j + 1) == finish {
                return Ok(step + 1);
            }
            q.push_back((i, j + 1, step + 1));
        }
    }
    Err(Error::EndLoop)
}

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<i32, Error> {
    let mut grid = parse_input(lines)?;
    let n = grid.len();
    let m = grid[0].len();
    play(&mut grid, (0, 1), (n - 1, m - 2))
}

pub fn task2<S: AsRef<str>>(lines: &[S]) -> Result<i32, Error> {
    let mut grid = parse_input(lines)?;
    let n = grid.len();
    let m = grid[0].len();
    let a = play(&mut grid, (0, 1), (n - 1, m - 2))?;
    let b = play(&mut grid, (n - 1, m - 2), (0, 1))?;
    let c = play(&mut grid, (0, 1), (n - 1, m - 2))?;
    Ok(a + b + c)
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str =
"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    #[test]
    fn test1() {
        let lines = DATA.split('\n').collect::<Vec<_>>();
        assert_eq!(Ok(18), task1(&lines));
    }

    #[test]
    fn test2() {
        let lines = DATA.split('\n').map(|s| s.trim()).collect::<Vec<_>>();
        assert_eq!(Ok(54), task2(&lines));
    }
}
