mod input;
use input::{read_file, read_stdin};
use std::io;
use std::env;
use std::num::ParseIntError;

enum Mode {
    File(String),
    Stdin
}

fn is_contain(r1: (u32, u32), r2: (u32, u32)) -> bool {
    r1.0 <= r2.0 && r1.1 >= r2.1
}

fn is_intersect(r1: (u32, u32), r2: (u32, u32)) -> bool {
    (r1.0 <= r2.0 && r2.0 <= r1.1) || (r1.0 <= r2.1 && r2.1 <= r1.1)
}

fn split_range(s: &str) -> Result<((u32, u32), (u32, u32)), ParseIntError> {
    let (s1, s2) = s.split_once(',').unwrap();
    let r1 = s1.split_once('-').unwrap();
    let r2 = s2.split_once('-').unwrap();
    Ok(((r1.0.parse()?, r1.1.parse()?), (r2.0.parse()?, r2.1.parse()?)))
}

fn overlaps<F>(data: &[String], compare: F) -> Result<i32, ParseIntError>
where F: Fn((u32, u32), (u32, u32)) -> bool {
    let mut total = 0;
    for row in data {
        let (range1, range2) = split_range(row).unwrap();
        if compare(range1, range2) || compare(range2, range1) {
            total += 1;
        }
    }
    Ok(total)
}

fn main() -> io::Result<()>{
    let mode = env::args().nth(1).map(|arg| Mode::File(arg)).unwrap_or(Mode::Stdin);
    let data = match mode {
        Mode::File(file_path) => read_file(&file_path)?,
        Mode::Stdin => read_stdin()?
    };
    match overlaps(&data, is_contain) {
        Ok(result) => println!("result {}", result),
        Err(error) => println!("error {}", error)
    }
    match overlaps(&data, is_intersect) {
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
        let data = &[
            "2-4,6-8",
            "2-3,4-5",
            "5-7,7-9",
            "2-8,3-7",
            "6-6,4-6",
            "2-6,4-8",
        ].into_iter().map(|s| s.to_string()).collect::<Vec<String>>();
        assert_eq!(Ok(2), overlaps(&data, is_contain));
    }
    #[test]
    fn test2() {
        let data = &[
            "2-4,6-8",
            "2-3,4-5",
            "5-7,7-9",
            "2-8,3-7",
            "6-6,4-6",
            "2-6,4-8",
        ].into_iter().map(|s| s.to_string()).collect::<Vec<String>>();
        assert_eq!(Ok(4), overlaps(&data, is_intersect));
    }

}
