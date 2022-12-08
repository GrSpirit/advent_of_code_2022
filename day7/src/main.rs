mod input;
use input::{read_file, read_stdin};
use std::io;
use std::env;
use std::str::FromStr;
use std::collections::BTreeMap;

enum Mode {
    File(String),
    Stdin
}

#[derive(Debug)]
enum CommandLine {
    List,
    Cd(String),
    Dir(String),
    File(String, u32),
}

impl FromStr for CommandLine {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<_>>();
        if parts[0] == "$" {
            match parts[1] {
                "ls" => Ok(CommandLine::List),
                "cd" => Ok(CommandLine::Cd(parts[2].to_string())),
                _ => unreachable!()
            }
        } else {
            match parts[0] {
                "dir" => Ok(CommandLine::Dir(parts[1].to_string())),
                x => Ok(CommandLine::File(parts[1].to_string(), x.parse::<u32>().unwrap()))
            }
        }
    }
}

fn task1(lines: &[String]) -> Result<u32, &'static str> {
    let mut path = vec![String::from("/")];
    let mut sizes = BTreeMap::new();
    let make_dir = |path: &[String]| -> String {path.iter().map(|s| s.as_str()).collect()};
    for line in lines {
        let cmd = line.parse::<CommandLine>().unwrap();
        match cmd {
            CommandLine::Cd(dir) => {
                match dir.as_str() {
                    ".." => {path.pop().unwrap();},
                    "/" => {path = vec![String::from("/")];},
                    name => {path.push(format!("{}/", name));},
                }
            },
            CommandLine::List => {
            },
            CommandLine::Dir(_name) => {},
            CommandLine::File(_name, size) => {
                for i in 1..=path.len() {
                    let dir_name = make_dir(&path[..i]);
                    *sizes.entry(dir_name).or_insert(0u32) += size;
                }
            }
        }
    }

    Ok(sizes.into_values().filter(|size| size <= &100000).sum())
}

fn task2(lines: &[String]) -> Result<u32, &'static str> {
    let max_size = 70000000 - 30000000;
    let mut path = vec![String::from("/")];
    let mut sizes: BTreeMap<String, u32> = BTreeMap::new();
    let make_dir = |path: &[String]| -> String {path.iter().map(|s| s.as_str()).collect()};
    for line in lines {
        let cmd = line.parse::<CommandLine>().unwrap();
        match cmd {
            CommandLine::Cd(dir) => {
                match dir.as_str() {
                    ".." => {path.pop().unwrap();},
                    "/" => {path = vec![String::from("/")];},
                    name => {path.push(format!("{}/", name));},
                }
            },
            CommandLine::List => {
            },
            CommandLine::Dir(_name) => {},
            CommandLine::File(_name, size) => {
                for i in 1..=path.len() {
                    let dir_name = make_dir(&path[..i]);
                    *sizes.entry(dir_name).or_default() += size;
                }
            }
        }
    }

    let total_size = *sizes.get("/").unwrap();
    let need_size = total_size - max_size;
    Ok(sizes.into_values().filter(|x| x >= &need_size).min().unwrap())
}

fn main() -> io::Result<()>{
    let mode = env::args().nth(1).map(|arg| Mode::File(arg)).unwrap_or(Mode::Stdin);
    let data = match mode {
        Mode::File(file_path) => read_file(&file_path)?,
        Mode::Stdin => read_stdin()?
    };
    match task1(&data) {
        Ok(result) => println!("result {:?}", result),
        Err(error) => println!("error {}", error)
    }
    match task2(&data) {
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
            "$ cd /",
            "$ ls",
            "dir a",
            "14848514 b.txt",
            "8504156 c.dat",
            "dir d",
            "$ cd a",
            "$ ls",
            "dir e",
            "29116 f",
            "2557 g",
            "62596 h.lst",
            "$ cd e",
            "$ ls",
            "584 i",
            "$ cd ..",
            "$ cd ..",
            "$ cd d",
            "$ ls",
            "4060174 j",
            "8033020 d.log",
            "5626152 d.ext",
            "7214296 k",
        ].into_iter().map(|s| s.to_string()).collect::<Vec<String>>();
        assert_eq!(Ok(95437), task1(&data));
        assert_eq!(Ok(24933642), task2(&data));
    }
}
