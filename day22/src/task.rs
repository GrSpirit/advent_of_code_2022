#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
    #[error("Internal error")]
    #[allow(unused)]
    Internal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Field {
    Empty,
    Space,
    Wall,
}

#[derive(Debug, Clone, Copy)]
enum Path {
    Forward(usize),
    Left,
    Right,
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

fn parse_path(path: &str) -> Vec<Path> {
    let mut result = Vec::new();
    let bytes = path.as_bytes();
    let mut p = 0;
    for i in 0..path.len() {
        if !bytes[i].is_ascii_digit() {
            if i > p {
                result.push(Path::Forward(path[p..i].parse().unwrap()));
            }
            if bytes[i] == b'L' {
                result.push(Path::Left);
            } else if bytes[i] == b'R' {
                result.push(Path::Right);
            }
            p = i + 1;
        }
    }
    if p < path.len() {
        result.push(Path::Forward(path[p..].parse().unwrap()));
    }
    result
}

fn parse_input<S: AsRef<str>>(lines: &[S]) -> Result<(Vec<Vec<Field>>, Vec<Path>), Error> {
    let mut raw_grid = Vec::new();
    let mut max_width = 0;
    for line in lines {
        raw_grid.push(line.as_ref());
        max_width = max_width.max(line.as_ref().len());
        if line.as_ref().is_empty() {
            break;
        }
    }
    let grid = raw_grid.into_iter().map(|row| {
        let mut r = row.bytes().map(|b| match b {
            b' ' => Field::Empty,
            b'.' => Field::Space,
            b'#' => Field::Wall,
            _ => unreachable!()
        }).collect::<Vec<_>>();
        if r.len() < max_width {
            let n = max_width - r.len();
            r.extend(vec![Field::Empty; n]);
        }
        r
    }).collect::<Vec<_>>();
    Ok((grid, parse_path(lines.last().unwrap().as_ref())))
}

fn print_grid(grid: &Vec<Vec<Field>>, position: (usize, usize), dir: Direction) {
    return;
    for (i, row) in grid.iter().enumerate() {
        for (j, field) in row.iter().enumerate() {
            if (i, j) == position {
                match dir {
                    Direction::Right => print!(">"),
                    Direction::Left => print!("<"),
                    Direction::Up => print!("^"),
                    Direction::Down => print!("v"),
                }
            } else {
                match field {
                    Field::Empty => print!(" "),
                    Field::Space => print!("."),
                    Field::Wall => print!("#"),
                }
            }
        }
        println!();
    }
    println!();
}

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<i32, Error> {
    fn move_to(grid: &Vec<Vec<Field>>, mut position: (usize, usize), dir: Direction) -> Option<(usize, usize)> {
        let n = grid.len();
        let m = grid[0].len();
        loop {
            match dir {
                Direction::Right => {
                    if position.1 == m - 1 {
                        position.1 = 0;
                    } else {
                        position.1 += 1;
                    }
                },
                Direction::Left => {
                    if position.1 == 0 {
                        position.1 = m - 1;
                    } else {
                        position.1 -= 1;
                    }
                },
                Direction::Up => {
                    if position.0 == 0 {
                        position.0 = n - 1;
                    } else {
                        position.0 -= 1;
                    }
                },
                Direction::Down => {
                    if position.0 == n - 1 {
                        position.0 = 0;
                    } else {
                        position.0 += 1;
                    }
                }
            }
            match grid[position.0][position.1] {
                Field::Wall => return None,
                Field::Space => return Some(position),
                _ => {} // continue
            }
        }
    }
    let (grid, path) = parse_input(lines)?;
    let mut dir = Direction::Right;
    let mut position = (0usize, grid[0].iter().position(|f| *f == Field::Space).unwrap());
    print_grid(&grid, position, dir);
    for p in path {
        match p {
            Path::Forward(n) => {
                for _ in 0..n {
                    if let Some(next_position) = move_to(&grid, position, dir) {
                        position = next_position;
                    } else {
                        break;
                    }
                    print_grid(&grid, position, dir);
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
        print_grid(&grid, position, dir);
    }
    println!("finish at {:?} {:?}", position, dir);
    let res = (position.0 as i32 + 1) * 1000 + (position.1 as i32 + 1) * 4 + i32::from(dir);
    Ok(res)
}

pub fn task2<S: AsRef<str>>(_lines: &[S]) -> Result<i32, Error> {
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
}
