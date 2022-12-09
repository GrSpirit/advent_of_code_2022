mod input;
use input::{read_file, read_stdin};
use std::io;
use std::env;
use std::collections::HashSet;

enum Mode {
    File(String),
    Stdin
}

fn task(lines: &[String], len: usize) -> Result<u32, &'static str> {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut rope = vec![(0, 0); len];
    visited.insert((0, 0));
    for line in lines {
        let (direction, ns) = line.split_once(' ').unwrap();
        let n = ns.parse().unwrap();
        for _ in 0..n {
            match direction {
                "U" => {
                    rope[0].1 += 1;
                },
                "D" => {
                    rope[0].1 -= 1;
                },
                "L" => {
                    rope[0].0 -= 1;
                },
                "R" => {
                    rope[0].0 += 1;
                },
                _ => return Err("illegal direction")
            }
            for i in 1..len {
                let diff = (rope[i - 1].0 - rope[i].0, rope[i - 1].1 - rope[i].1);
                match diff {
                    (2, 0) => {
                        rope[i].0 += 1;
                    },
                    (-2, 0) => {
                        rope[i].0 -= 1;
                    },
                    (0, 2) => {
                        rope[i].1 += 1;
                    },
                    (0, -2) => {
                        rope[i].1 -= 1;
                    },
                    (2, 1) | (1, 2) | (2, 2) => {
                        rope[i].0 += 1;
                        rope[i].1 += 1;
                    },
                    (-2, -1) | (-1, -2) | (-2, -2) => {
                        rope[i].0 -= 1;
                        rope[i].1 -= 1;
                    }
                    (2, -1) | (1, -2) | (2, -2) => {
                        rope[i].0 += 1;
                        rope[i].1 -= 1;
                    },
                    (-2, 1) | (-1, 2) | (-2, 2) => {
                        rope[i].0 -= 1;
                        rope[i].1 += 1;
                    },
                    _ => {
                        break;
                    }
                }
                if i == len - 1 {
                    visited.insert(*rope.last().unwrap());
                }
            }
        }
    }
    Ok(visited.len() as u32)
}

fn main() -> io::Result<()>{
    let mode = env::args().nth(1).map(|arg| Mode::File(arg)).unwrap_or(Mode::Stdin);
    let data = match mode {
        Mode::File(file_path) => read_file(&file_path)?,
        Mode::Stdin => read_stdin()?
    };
    match task(&data, 2) {
        Ok(result) => println!("result {:?}", result),
        Err(error) => println!("error {}", error)
    }
    match task(&data, 10) {
        Ok(result) => println!("result {:?}", result),
        Err(error) => println!("error {}", error)
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let data = &[
            "R 4",
            "U 4",
            "L 3",
            "D 1",
            "R 4",
            "D 1",
            "L 5",
            "R 2",
        ].into_iter().map(|s| s.to_string()).collect::<Vec<String>>();
        assert_eq!(Ok(13), task(&data, 2));
    }
    #[test]
    fn test2() {
        let data = &[
            "R 5",
            "U 8",
            "L 8",
            "D 3",
            "R 17",
            "D 10",
            "L 25",
            "U 20",
        ].into_iter().map(|s| s.to_string()).collect::<Vec<String>>();
        assert_eq!(Ok(36), task(&data, 10));
    }
}
