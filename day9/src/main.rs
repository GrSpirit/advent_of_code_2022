mod input;
use input::{read_file, read_stdin};
use std::io;
use std::env;

enum Mode {
    File(String),
    Stdin
}

fn task1(lines: &[String]) -> Result<u32, &'static str> {
    Ok(lines.len() as u32)
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
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let data = &[
            "30373",
            "25512",
            "65332",
            "33549",
            "35390",
        ].into_iter().map(|s| s.to_string()).collect::<Vec<String>>();
        assert_eq!(Ok(5), task1(&data));
    }
}
