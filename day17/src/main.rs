mod input;
mod task;
use input::{read_file, read_stdin};
use std::io;
use std::env;
use task::*;

enum Mode {
    File(String),
    Stdin
}

fn main() -> io::Result<()>{
    let mode = env::args().nth(1).map(|arg| Mode::File(arg)).unwrap_or(Mode::Stdin);
    let data = match mode {
        Mode::File(file_path) => read_file(&file_path)?,
        Mode::Stdin => read_stdin()?
    };
    match task1(&data, 2022) {
        Ok(result) => println!("result {}", result),
        Err(error) => println!("error {}", error)
    }
    let x = 1000000000000 - 1739;
    match task1(&data, 1739 + x % 1730) {
        Ok(result) => println!("result {}", result + x / 1730 * 2659),
        Err(error) => println!("error {}", error)
    }
    Ok(())
}
