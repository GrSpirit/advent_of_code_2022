mod input;
use input::read_file;
use std::io;
use std::env;
use std::collections::HashSet;

enum Mode {
    File(String),
    Stdin
}

fn priority_map(b: u8) -> u32 {
    match b {
        b'a'..=b'z' =>(b - b'a' + 1) as u32,
        b'A'..=b'Z' => (b - b'A' + 27) as u32,
        _ => unreachable!()
    }
}

fn unique_chars(s: &[u8]) -> HashSet<u8> {
    s.iter().cloned().collect()
}

fn intersect(a: HashSet<u8>, b: HashSet<u8>) -> HashSet<u8> {
    a.intersection(&b).cloned().collect()
}

fn compare(a: &[u8], b: &[u8]) -> Option<u8> {
    intersect(unique_chars(a), unique_chars(b)).into_iter().next()
}

fn process(data: &[String]) -> Result<u32, &'static str> {
    let mut total = 0;
    for s in data {
        let row = s.as_bytes();
        let m = row.len() / 2;
        let uniq = compare(&row[..m], &row[m..]).ok_or("uniq char not found")?;
        total += priority_map(uniq);
    }
    Ok(total)
}

fn count_badges(data: &[String]) -> Result<u32, &'static str> {
    let mut total = 0;
    for parts in data.chunks(3) {
        let h1 = intersect(unique_chars(parts[0].as_bytes()), unique_chars(parts[1].as_bytes()));
        let h2 = intersect(h1, unique_chars(parts[2].as_bytes()));
        if h2.len() > 1 {
            return Err("too many items after intersection");
        }
        total += priority_map(h2.into_iter().next().ok_or("set is empty")?);
    }
    Ok(total)
}

fn main() -> io::Result<()>{
    let mode = env::args().nth(1).map(|arg| Mode::File(arg)).unwrap_or(Mode::Stdin);
    let data = match mode {
        Mode::File(file_path) => read_file(&file_path)?,
        _ => panic!("not implemented")
    };
    match process(&data) {
        Ok(result) => println!("result {}", result),
        Err(error) => println!("error {}", error)
    }
    match count_badges(&data) {
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
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ].into_iter().map(|s| s.to_string()).collect::<Vec<String>>();
        assert_eq!(Ok(157), process(&data));
    }
    #[test]
    fn test2() {
        let data = &[
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ].into_iter().map(|s| s.to_string()).collect::<Vec<String>>();
        assert_eq!(Ok(70), count_badges(&data));
    }

}
