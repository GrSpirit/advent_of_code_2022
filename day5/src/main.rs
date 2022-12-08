mod input;
use input::{read_file, process_input};
use std::io;
use std::env;

enum Mode {
    File(String),
    Stdin
}

fn task1(mut stack: Vec<Vec<u8>>, moves: &[(usize, usize, usize)]) -> Result<String, &'static str> {
    for (count, from, to) in moves {
        for _ in 0..*count {
            if !stack[*from].is_empty() {
                let val = stack[*from].pop().unwrap();
                stack[*to].push(val);
            }
        }
    }
    Ok(String::from_utf8(stack.into_iter().map(|s| *s.last().unwrap_or(&0x20)).collect::<Vec<u8>>()).unwrap())
}

fn task2(mut stack: Vec<Vec<u8>>, moves: &[(usize, usize, usize)]) -> Result<String, &'static str> {
    for (count, from, to) in moves {
        let mut tmp = Vec::new();
        for _ in 0..*count {
            if !stack[*from].is_empty() {
                let val = stack[*from].pop().unwrap();
                tmp.push(val);
            }
        }
        for _ in 0..*count {
            let val = tmp.pop().unwrap();
            stack[*to].push(val);
        }
    }
    Ok(String::from_utf8(stack.into_iter().map(|s| *s.last().unwrap_or(&0x20)).collect::<Vec<u8>>()).unwrap())
}

fn main() -> io::Result<()>{
    let mode = env::args().nth(1).map(|arg| Mode::File(arg)).unwrap_or(Mode::Stdin);
    let (stack, moves) = match mode {
        Mode::File(file_path) => process_input(read_file(&file_path)?),
        Mode::Stdin => panic!("not implemented"),
    };
    match task1(stack.clone(), &moves) {
        Ok(result) => println!("result {}", result),
        Err(error) => println!("error {}", error)
    }
    match task2(stack, &moves) {
        Ok(result) => println!("result {}", result),
        Err(error) => println!("error {}", error)
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let data = vec![
            "[D]        ",
            "[N] [C]    ",
            "[Z] [M] [P]",
            " 1   2   3 ",
            "",
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ].into_iter().map(|s| s.to_string()).collect::<Vec<String>>();
        let (stack, moves) = process_input(data);
        assert_eq!(Ok("ZMN".to_string()), task1(stack, &moves));
    }
}
