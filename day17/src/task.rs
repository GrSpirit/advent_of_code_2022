use std::{mem::swap, fmt::Display};
use lazy_static::lazy_static;

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
    #[error("Internal Error")]
    #[allow(unused)]
    Internal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Left,
    Right,
}

lazy_static! {
    static ref SHAPES: Vec<Vec<Vec<u8>>> = vec![
        vec![vec![2, 0, 0, 1, 1, 1, 1, 0, 2]],
        vec![
            vec![2, 0, 0, 0, 1, 0, 0, 0, 2],
            vec![2, 0, 0, 1, 1, 1, 0, 0, 2],
            vec![2, 0, 0, 0, 1, 0, 0, 0, 2],
        ],
        vec![
            vec![2, 0, 0, 0, 0, 1, 0, 0, 2],
            vec![2, 0, 0, 0, 0, 1, 0, 0, 2],
            vec![2, 0, 0, 1, 1, 1, 0, 0, 2],
        ],
        vec![
            vec![2, 0, 0, 1, 0, 0, 0, 0, 2],
            vec![2, 0, 0, 1, 0, 0, 0, 0, 2],
            vec![2, 0, 0, 1, 0, 0, 0, 0, 2],
            vec![2, 0, 0, 1, 0, 0, 0, 0, 2],
        ],
        vec![
            vec![2, 0, 0, 1, 1, 0, 0, 0, 2],
            vec![2, 0, 0, 1, 1, 0, 0, 0, 2],
        ]
    ];
}

#[derive(Debug)]
struct Game {
    board: Vec<Vec<u8>>,
    new_board: Vec<Vec<u8>>,
}

impl Game {
    fn new() -> Self {
        let board = vec![vec![2; 9]];
        let new_board = vec![vec![2; 9]];
        Self { board, new_board }
    }
    fn clear_board(board: &mut Vec<Vec<u8>>) {
        board.iter_mut().for_each(|row| row.iter_mut().for_each(|x| if *x == 1 { *x = 0; }));
    }
    fn shift(&mut self, direction: Dir) {
        #[derive(Debug, PartialEq, Eq, Clone, Copy)]
        enum ShiftState {
            Search,
            Found,
            Finish,
        }
        let prev = match direction {
            Dir::Left => |x| x + 1,
            Dir::Right => |x| x - 1,
        };
        let range = match direction {
            Dir::Left => 0..8,
            Dir::Right => 1..9,
        };
        Self::clear_board(&mut self.new_board);
        let mut state = ShiftState::Search;
        for i in (0..self.board.len()).rev() {
            let mut found = false;
            for j in range.clone() {
                if self.board[i][prev(j)] == 1 {
                    if self.board[i][j] == 2 {
                        return;
                    }
                    self.new_board[i][j] = 1;
                    found = true;
                }
            }
            state = if found {
                match state {
                    ShiftState::Search | ShiftState::Found => ShiftState::Found,
                    ShiftState::Finish => unreachable!()
                }
            } else {
                match state {
                    ShiftState::Search => ShiftState::Search,
                    ShiftState::Found => ShiftState::Finish,
                    ShiftState::Finish => unreachable!()
                }
            };
            if state == ShiftState::Finish {
                break;
            }
        }
        swap(&mut self.new_board, &mut self.board);
    }
    fn drop(&mut self) -> bool {
        Self::clear_board(&mut self.new_board);
        for i in (0..self.board.len()).rev() {
            for j in 1..8 {
                if self.board[i][j] == 1 {
                    if i == 0 || self.board[i - 1][j] == 2 {
                        return false;
                    }
                    self.new_board[i - 1][j] = self.board[i][j];
                }
            }
        }
        swap(&mut self.new_board, &mut self.board);
        true
    }
    fn freeze(&mut self) {
        for i in 0..self.board.len() {
            for j in 1..8 {
                if self.board[i][j] == 1 {
                    self.board[i][j] = 2;
                    self.new_board[i][j] = 2;
                }
            }
        }
    }
    fn add_shape(&mut self, n: usize) {
        let shape = &SHAPES[n];
        let board_height = self.len();
        let shape_height = shape.len();
        let new_board_height = board_height + 3 + shape_height;
        let mut h = self.board.len();
        if h > new_board_height {
            self.board.resize(new_board_height, vec![]);
            self.new_board.resize(new_board_height, vec![])
        } else {
            for _ in 0..(new_board_height - h) {
                self.board.push(vec![2, 0, 0, 0, 0, 0, 0, 0, 2]);
                self.new_board.push(vec![2, 0, 0, 0, 0, 0, 0, 0, 2]);
            }
        }
        h = self.board.len();
        let shape_len = shape.len();
        for i in 0..shape_len {
            self.board[h - i - 1] = shape[i].clone();
        }
        if h > 10000 {
            self.board.drain(..9000);
            self.new_board.drain(..9000);
        }
    }
    fn play(&mut self, pattern: &[Dir], mut n: usize) {
        let mut p = 0;
        let mut s = 1;
        self.add_shape(0);
        loop {
            self.shift(pattern[p]);
            if !self.drop() {
                self.freeze();
                n -= 1;
                if n == 0 {
                    break;
                }
                self.add_shape(s);
                s = (s + 1) % SHAPES.len();
            }
            p = (p + 1) % pattern.len();
        }
    }
    fn len(&self) -> usize {
        self.board.iter().rposition(|row| row.iter().skip(1).take(7).any(|x| x == &2)).unwrap_or_default() + 1
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in (0..self.board.len()).rev() {
            let s1 = self.board[i].iter().map(|x| match *x {1 => '@', 2 => '#', _ => '.' }).collect::<String>();
            let s2 = self.new_board[i].iter().map(|x| match *x {1 => '@', 2 => '#', _ => '.' }).collect::<String>();
            writeln!(f, "{} {}", s1, s2)?;
        }
        Ok(())
    }
}

pub fn task1<S: AsRef<str>>(lines: &[S], n: usize) -> Result<usize, Error> {
    let mut game = Game::new();
    let pattern = lines[0].as_ref().bytes().map(|b| if b == b'<' { Dir::Left } else { Dir::Right }).collect::<Vec<_>>();
    game.play(&pattern, n);
    Ok(game.len() - 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &'static [&'static str] = &[">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"];

    #[test]
    fn test1() {
        assert_eq!(Ok(3069), task1(&DATA, 2022));
    }
}
