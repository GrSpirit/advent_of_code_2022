use std::sync::mpsc::Sender;
use std::io::{self, BufRead};
use std::fs::File;

pub fn read_stdin(sender: Sender<String>) -> io::Result<()> {
    let mut lines = io::stdin().lock().lines();
    while let Some(line) = lines.next() {
        let input = line?;
        sender.send(input).unwrap();
    }
    drop(sender);
    Ok(())
}

pub fn read_file(sender: Sender<String>, file_name: String) -> io::Result<()> {
    let file = File::open(file_name)?;
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        sender.send(line?).unwrap();
    }
    Ok(())
}
