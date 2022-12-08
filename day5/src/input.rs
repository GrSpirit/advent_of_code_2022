use std::io::{self, BufRead};
use std::fs::File;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Stage {
    Stack,
    Moves,
}

fn transpose(input: &Vec<String>, size: usize) -> Vec<Vec<u8>> {
    let mut result = vec![Vec::new(); size];
    for row in input.iter().rev() {
        let r = row.as_bytes();
        for i in 0..size {
            if r[i*4 + 1] != b' ' {
                result[i].push(r[i*4 + 1]);
            }
        }
    }
    result
}


pub fn read_file(file_name: &str) -> io::Result<Vec<String>> {
    let mut result = Vec::new();
    let file = File::open(file_name)?;
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        result.push(line?);
    }
    Ok(result)
}

pub fn process_input(input: Vec<String>) -> (Vec<Vec<u8>>, Vec<(usize, usize, usize)>) {
    let mut stage = Stage::Stack;
    let mut holder: Vec<String> = Vec::new();
    let mut stack = Vec::new();
    let mut moves = Vec::new();
    for row in input {
        match stage {
            Stage::Stack => {
                if row.is_empty() {
                    let count_line = holder.pop().unwrap();
                    let count = count_line.split_ascii_whitespace().last().unwrap().parse::<usize>().unwrap();
                    stack = transpose(&holder, count);
                    stage = Stage::Moves;
                } else {
                    holder.push(row);
                }
            },
            Stage::Moves => {
                let parts = row.split_ascii_whitespace().collect::<Vec<_>>();
                moves.push((parts[1].parse::<usize>().unwrap(), parts[3].parse::<usize>().unwrap() - 1, parts[5].parse::<usize>().unwrap() - 1));
            }
        }
    }
    (stack, moves)
}

pub fn read_stdin() -> io::Result<Vec<String>> {
    io::stdin().lock().lines().collect()
}
