use std::convert::TryFrom;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    West,
    East
}

fn parse_input<S: AsRef<str>>(lines: &[S]) -> Vec<Vec<Field>> {
    let initial_grid = lines.iter().map(|row| row.as_ref().bytes().map(|b| Field::try_from(b).unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();
    let n = initial_grid.len();
    let m = initial_grid[0].len();
    let mut grid = vec![vec![Field::Empty; m]; 2*n];
    grid.extend_from_slice(&initial_grid);
    grid.extend_from_slice(&vec![vec![Field::Empty; m]; 2*n]);
    grid.into_iter().map(|initial_row| {
        let mut row = vec![Field::Empty; 2*m];
        row.extend_from_slice(&initial_row);
        row.extend_from_slice(&vec![Field::Empty; 2*m]);
        row
    }).collect()
}

fn range(x: usize, n: usize) -> (usize, usize) {
    ((x as i32 - 1).max(0) as usize, (x + 1).min(n - 1))
}

fn try_direction(grid: &Vec<Vec<Field>>, d: Direction, i: usize, j: usize) -> bool {
    let n = grid.len();
    let m = grid[0].len();
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

fn is_alone(grid: &Vec<Vec<Field>>, ii: usize, jj: usize) -> bool {
    let n = grid.len();
    let m = grid[0].len();
    let (i_from, i_to) = range(ii, n);
    let (j_from, j_to) = range(jj, m);
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

fn try_move(grid: &Vec<Vec<Field>>, proposal: &Vec<Vec<Option<Direction>>>, d: Direction, ii: usize, jj: usize) -> Option<(usize, usize)> {
    let (i, j) = match d {
        Direction::North => (ii - 1, jj),
        Direction::South => (ii + 1, jj),
        Direction::East => (ii, jj + 1),
        Direction::West => (ii, jj - 1),
    };
    let n = grid.len();
    let m = grid[0].len();
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

fn print_grid(grid: &Vec<Vec<Field>>) {
    grid.iter().for_each(|row| {
        row.iter().for_each(|f| {
            match *f {
                Field::Empty => print!("."),
                Field::Occupied => print!("#"),
            }
        });
        println!();
    });
    println!();
}

fn play(grid: &mut Vec<Vec<Field>>, tmp_grid: &mut Vec<Vec<Field>>, round: usize) -> bool {
    let directions = [Direction::North, Direction::South, Direction::West, Direction::East];
    let n = grid.len();
    let m = grid[0].len();
    tmp_grid.iter_mut().for_each(|row| row.iter_mut().for_each(|f| *f = Field::Empty));
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
    swap(grid, tmp_grid);
    moves
}

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<u32, Error> {
    let mut grid = parse_input(lines);
    let mut tmp_grid = vec![vec![Field::Empty; grid[0].len()]; grid.len()];
    for r in 0..10 {
        play(&mut grid, &mut tmp_grid, r);
    }

    let mut i_from = 0;
    loop {
        let is_empty = grid[i_from].iter().all(|&f| f == Field::Empty);
        if !is_empty { break }
        i_from += 1;
    }
    let mut i_to = grid.len() - 1;
    loop {
        let is_empty = grid[i_to].iter().all(|&f| f == Field::Empty);
        if !is_empty { break }
        i_to -= 1;
    }
    let mut j_from = 0;
    loop {
        let is_empty = grid.iter().all(|r| r[j_from] == Field::Empty);
        if !is_empty { break }
        j_from += 1;
    }
    let mut j_to = grid[0].len() - 1;
    loop {
        let is_empty = grid.iter().all(|r| r[j_to] == Field::Empty);
        if !is_empty { break }
        j_to -= 1;
    }
    Ok(grid.into_iter().skip(i_from).take(i_to - i_from + 1).map(
        |row| row.into_iter().skip(j_from).take(j_to - j_from + 1).filter(|&f| f == Field::Empty).count()
    ).sum::<usize>() as u32)
}

pub fn task2<S: AsRef<str>>(lines: &[S]) -> Result<usize, Error> {
    let mut grid = parse_input(lines);
    let mut tmp_grid = vec![vec![Field::Empty; grid[0].len()]; grid.len()];
    for r in 0.. {
        if !play(&mut grid, &mut tmp_grid, r) {
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
