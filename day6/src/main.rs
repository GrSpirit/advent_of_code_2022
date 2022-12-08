mod input;
use input::{read_file, read_stdin};
use std::io;
use std::env;
use std::collections::HashSet;

enum Mode {
    File(String),
    Stdin
}

fn is_uniq(s: &[u8]) -> bool {
    let mut set = HashSet::new();
    s.iter().all(|b| set.insert(b))
}

fn find_mark(s: &str, n: usize) -> usize {
    let row = s.as_bytes();
    row.windows(n).enumerate().find(|(_, v)| is_uniq(v)).map(|(i, _)| i + n).unwrap()
}

fn task1(data: &[String], n: usize) -> Result<Vec<usize>, &'static str> {
    Ok(data.iter().map(|s| find_mark(s, n)).collect())
}

fn main() -> io::Result<()>{
    let mode = env::args().nth(1).map(|arg| Mode::File(arg)).unwrap_or(Mode::Stdin);
    let data = match mode {
        Mode::File(file_path) => read_file(&file_path)?,
        Mode::Stdin => read_stdin()?
    };
    match task1(&data, 4) {
        Ok(result) => println!("result {:?}", result),
        Err(error) => println!("error {}", error)
    }
    match task1(&data, 14) {
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
            "bvwbjplbgvbhsrlpgdmjqwftvncz",
            "nppdvjthqldpwncqszvftbrmjlhg",
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
        ].into_iter().map(|s| s.to_string()).collect::<Vec<String>>();
        let res = task1(&data, 4).unwrap();
        assert_eq!(5, res[0]);
        assert_eq!(6, res[1]);
        assert_eq!(10, res[2]);
        assert_eq!(11, res[3]);
    }
    #[test]
    fn test2() {
        let data = &[
            "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
            "bvwbjplbgvbhsrlpgdmjqwftvncz",
            "nppdvjthqldpwncqszvftbrmjlhg",
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
        ].into_iter().map(|s| s.to_string()).collect::<Vec<String>>();
        let res = task1(&data, 14).unwrap();
        assert_eq!(19, res[0]);
        assert_eq!(23, res[1]);
        assert_eq!(23, res[2]);
        assert_eq!(29, res[3]);
        assert_eq!(26, res[4]);
    }
}
