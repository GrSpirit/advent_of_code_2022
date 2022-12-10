mod input;
use input::{read_file, read_stdin};
use std::io;
use std::env;

enum Mode {
    File(String),
    Stdin
}

fn task(lines: &[String]) -> Result<i32, &'static str> {
    let mut n = 0;
    let mut x = 1;
    let mut total = 0;
    let mut inc = |x| {
        n += 1;
        if n == 20 || n == 60 || n == 100 || n == 140 || n == 180 || n == 220 {
            total += n * x;
        }
    };
    for line in lines {
        let cmd = line.split_ascii_whitespace().collect::<Vec<_>>();
        match cmd[0] {
            "noop" => {
                inc(x);
            },
            "addx" => {
                inc(x);
                inc(x);
                x += cmd[1].parse::<i32>().unwrap();
            },
            _ => return Err("invalid input")
        }
    }
    Ok(total)
}

fn task2(lines: &[String]) -> Result<(), &'static str> {
    let mut n = 0;
    let mut x = 0;
    let mut inc = |x| {
        let pixel = if n >= x && n <= x + 2 { '#' } else { '.' };
        print!("{}", pixel);
        n += 1;
        if (n % 40) == 0 {
            println!();
            n = 0;
        }
    };
    for line in lines {
        let cmd = line.split_ascii_whitespace().collect::<Vec<_>>();
        match cmd[0] {
            "noop" => {
                inc(x);
            },
            "addx" => {
                inc(x);
                inc(x);
                x += cmd[1].parse::<i32>().unwrap();
            },
            _ => return Err("invalid input")
        }
    }
    Ok(())
}

fn main() -> io::Result<()>{
    let mode = env::args().nth(1).map(|arg| Mode::File(arg)).unwrap_or(Mode::Stdin);
    let data = match mode {
        Mode::File(file_path) => read_file(&file_path)?,
        Mode::Stdin => read_stdin()?
    };
    match task(&data) {
        Ok(result) => println!("result {}", result),
        Err(error) => println!("error {}", error)
    }
    match task2(&data) {
        Ok(_) => println!("Ok"),
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
            "noop",
            "addx 3",
            "addx -5",
        ].into_iter().map(|s| s.to_string()).collect::<Vec<String>>();
        assert_eq!(Ok(0), task(&data));
    }
    #[test]
    fn test2() {
        let data = &[
            "addx 15",
            "addx -11",
            "addx 6",
            "addx -3",
            "addx 5",
            "addx -1",
            "addx -8",
            "addx 13",
            "addx 4",
            "noop",
            "addx -1",
            "addx 5",
            "addx -1",
            "addx 5",
            "addx -1",
            "addx 5",
            "addx -1",
            "addx 5",
            "addx -1",
            "addx -35",
            "addx 1",
            "addx 24",
            "addx -19",
            "addx 1",
            "addx 16",
            "addx -11",
            "noop",
            "noop",
            "addx 21",
            "addx -15",
            "noop",
            "noop",
            "addx -3",
            "addx 9",
            "addx 1",
            "addx -3",
            "addx 8",
            "addx 1",
            "addx 5",
            "noop",
            "noop",
            "noop",
            "noop",
            "noop",
            "addx -36",
            "noop",
            "addx 1",
            "addx 7",
            "noop",
            "noop",
            "noop",
            "addx 2",
            "addx 6",
            "noop",
            "noop",
            "noop",
            "noop",
            "noop",
            "addx 1",
            "noop",
            "noop",
            "addx 7",
            "addx 1",
            "noop",
            "addx -13",
            "addx 13",
            "addx 7",
            "noop",
            "addx 1",
            "addx -33",
            "noop",
            "noop",
            "noop",
            "addx 2",
            "noop",
            "noop",
            "noop",
            "addx 8",
            "noop",
            "addx -1",
            "addx 2",
            "addx 1",
            "noop",
            "addx 17",
            "addx -9",
            "addx 1",
            "addx 1",
            "addx -3",
            "addx 11",
            "noop",
            "noop",
            "addx 1",
            "noop",
            "addx 1",
            "noop",
            "noop",
            "addx -13",
            "addx -19",
            "addx 1",
            "addx 3",
            "addx 26",
            "addx -30",
            "addx 12",
            "addx -1",
            "addx 3",
            "addx 1",
            "noop",
            "noop",
            "noop",
            "addx -9",
            "addx 18",
            "addx 1",
            "addx 2",
            "noop",
            "noop",
            "addx 9",
            "noop",
            "noop",
            "noop",
            "addx -1",
            "addx 2",
            "addx -37",
            "addx 1",
            "addx 3",
            "noop",
            "addx 15",
            "addx -21",
            "addx 22",
            "addx -6",
            "addx 1",
            "noop",
            "addx 2",
            "addx 1",
            "noop",
            "addx -10",
            "noop",
            "noop",
            "addx 20",
            "addx 1",
            "addx 2",
            "addx 2",
            "addx -6",
            "addx -11",
            "noop",
            "noop",
            "noop",
        ].into_iter().map(|s| s.to_string()).collect::<Vec<String>>();
        assert_eq!(Ok(13140), task(data));
    }
}
