use std::convert::TryFrom;
use std::fmt::Display;
use std::mem::swap;

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
    #[error("Internal error")]
    #[allow(unused)]
    Internal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Field {
    Empty,
    Occupied
}

impl TryFrom<u8> for Field {
    type Error = Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'.' => Ok(Field::Empty),
            b'#' => Ok(Field::Occupied),
            _ => Err(Error::Internal),
        }
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Field::Empty => write!(f, "."),
            Field::Occupied => write!(f, "#")
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    West,
    East
}

type Grid = Vec<Vec<Field>>;
macro_rules! n {
    ($grid: expr) => {
        $grid.len()
    };
}
macro_rules! m {
    ($grid: expr) => {
        $grid[0].len()
    };
}

fn parse_input<S: AsRef<str>>(lines: &[S]) -> Grid {
    let n = lines.len();
    let m = lines[0].as_ref().len();
    let grid_iter = lines.iter().map(|row|
        (0..2*m).map(|_| Field::Empty)
        .chain(row.as_ref().bytes().map(|b| Field::try_from(b).unwrap()))
        .chain((0..2*m).map(|_| Field::Empty))
        .collect::<Vec<_>>()
    );
    (0..2*n).map(|_| vec![Field::Empty; 5*m]).chain(grid_iter).chain((0..2*n).map(|_| vec![Field::Empty; 5*m])).collect()
}

fn range(x: usize, n: usize) -> (usize, usize) {
    ((x as i32 - 1).max(0) as usize, (x + 1).min(n - 1))
}

fn try_direction(grid: &Grid, d: Direction, i: usize, j: usize) -> bool {
    let n = n!(grid);
    let m = m!(grid);
    let (i_from, i_to) = range(i, n);
    let (j_from, j_to) = range(j, m);
    match d {
        Direction::North => {
            i > 0 && grid[i - 1][j_from..=j_to].iter().all(|f| *f == Field::Empty)
        },
        Direction::South => {
            i + 1 < n && grid[i + 1][j_from..=j_to].iter().all(|f| *f == Field::Empty)
        },
        Direction::West => {
            j > 0 && grid[i_from..=i_to].iter().all(|f| f[j - 1] == Field::Empty)
        },
        Direction::East => {
            j + 1 < m && grid[i_from..=i_to].iter().all(|f| f[j + 1] == Field::Empty)
        }
    }
}

fn is_alone(grid: &Grid, ii: usize, jj: usize) -> bool {
    let (i_from, i_to) = range(ii, n!(grid));
    let (j_from, j_to) = range(jj, m!(grid));
    for i in i_from..=i_to {
        for j in j_from..=j_to {
            if i == ii && j == jj {
                continue;
            }
            if grid[i][j] == Field::Occupied {
                return false;
            }
        }
    }
    return true;
}

fn try_move(grid: &Grid, proposal: &Vec<Vec<Option<Direction>>>, d: Direction, ii: usize, jj: usize) -> Option<(usize, usize)> {
    let (i, j) = match d {
        Direction::North => (ii - 1, jj),
        Direction::South => (ii + 1, jj),
        Direction::East => (ii, jj + 1),
        Direction::West => (ii, jj - 1),
    };
    let n = n!(grid);
    let m = m!(grid);
    if (i > 0 && i - 1 != ii && proposal[i - 1][j].map(|d| d == Direction::South).unwrap_or(false)) ||
        (i + 1 < n && i + 1 != ii && proposal[i + 1][j].map(|d| d == Direction::North).unwrap_or(false)) ||
        (j > 0 && j - 1 != jj && proposal[i][j - 1].map(|d| d == Direction::East).unwrap_or(false)) ||
        (j + 1 < m && j + 1 != jj && proposal[i][j + 1].map(|d| d == Direction::West).unwrap_or(false))
    {
        None
    } else {
        Some((i, j))
    }

}

#[allow(unused)]
fn print_grid(grid: &Grid) {
    grid.iter().for_each(|row| {
        row.iter().for_each(|f| { print!("{}", f); });
        println!();
    });
    println!();
}

fn play(grid: &mut Grid, round: usize) -> bool {
    let directions = [Direction::North, Direction::South, Direction::West, Direction::East];
    let n = n!(grid);
    let m = m!(grid);
    let mut tmp_grid = vec![vec![Field::Empty; m]; n];
    let mut proposal: Vec<Vec<Option<Direction>>> = vec![vec![None; m]; n];
    let mut moves = false;
    for i in 0..n {
        for j in 0..m {
            match grid[i][j] {
                Field::Occupied => {
                    if !is_alone(&grid, i, j) {
                        let next_direction = directions.iter().cycle().skip(round).take(4).find(
                            |&&d| try_direction(grid, d, i, j)
                        ).copied();
                        proposal[i][j] = next_direction;
                        moves = true;
                    }
                },
                Field::Empty => ()
            }
        }
    }
    for i in 0..n {
        for j in 0..m {
            match grid[i][j] {
                Field::Occupied => {
                    if let Some(prop) = proposal[i][j] {
                        if let Some((pi, pj)) = try_move(grid, &proposal, prop, i, j) {
                            tmp_grid[pi][pj] = Field::Occupied;
                        } else {
                            tmp_grid[i][j] = Field::Occupied;
                        }
                    } else {
                        tmp_grid[i][j] = Field::Occupied;
                    }
                },
                Field::Empty => ()
            }
        }
    }
    swap(grid, &mut tmp_grid);
    moves
}

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<u32, Error> {
    let mut grid = parse_input(lines);
    let n = n!(grid);
    let m = m!(grid);
    for r in 0..10 {
        play(&mut grid, r);
    }

    let i_from = grid.iter().position(|row| row.iter().any(|&f| f == Field::Occupied)).unwrap();
    let i_to = n - grid.iter().rev().position(|row| row.iter().any(|&f| f == Field::Occupied)).unwrap();

    let j_from = (0..m).position(|i| grid.iter().any(|row| row[i] == Field::Occupied)).unwrap();
    let j_to = m - (0..m).rev().position(|i| grid.iter().any(|row| row[i] == Field::Occupied)).unwrap();
    Ok(grid.into_iter().skip(i_from).take(i_to - i_from).map(
        |row| row.into_iter().skip(j_from).take(j_to - j_from).filter(|&f| f == Field::Empty).count()
    ).sum::<usize>() as u32)
}

pub fn task2<S: AsRef<str>>(lines: &[S]) -> Result<usize, Error> {
    let mut grid = parse_input(lines);
    for r in 0.. {
        if !play(&mut grid, r) {
            return Ok(r + 1);
        }
    }
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str =
"..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............";

    #[test]
    fn test1() {
        let lines = DATA.split('\n').collect::<Vec<_>>();
        assert_eq!(Ok(110), task1(&lines));
    }

    #[test]
    fn test2() {
        let lines = DATA.split('\n').map(|s| s.trim()).collect::<Vec<_>>();
        assert_eq!(Ok(20), task2(&lines));
    }
}
