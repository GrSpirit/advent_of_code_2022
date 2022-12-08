use std::io::{self, BufRead};
use std::fs::File;

pub fn read_file(file_name: &str) -> io::Result<Vec<String>> {
    let mut result = Vec::new();
    let file = File::open(file_name)?;
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        result.push(line?);
    }
    Ok(result)
}
