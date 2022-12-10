
use std::{ops::FnMut, str::FromStr};

enum Command {
    Noop,
    Add(i32),
}

impl FromStr for Command {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cmd = s.split_ascii_whitespace().collect::<Vec<_>>();
        match cmd[0] {
            "noop" => {
                Ok(Command::Noop)
            },
            "addx" => {
                let x = cmd[1].parse::<i32>().map_err(|_| "cannot parse a number")?;
                Ok(Command::Add(x))
            },
            _ => Err("invalid input")
        }
    }
}

fn process<F>(lines: &[String], mut exec: F) -> Result<(), &'static str>
where F: FnMut(i32) {
    let mut x = 0;
    for line in lines {
        match line.parse::<Command>()? {
            Command::Noop => {
                exec(x);
            },
            Command::Add(y) => {
                exec(x);
                exec(x);
                x += y;
            }
        }
    }
    Ok(())
}

pub fn task1(lines: &[String]) -> Result<i32, &'static str> {
    let mut n = 0;
    let mut total = 0;
    let inc = |x| {
        n += 1;
        if (n + 20) % 40 == 0 {
            total += n * (x + 1);
        }
    };
    process(lines, inc)?;
    Ok(total)
}

pub fn task2(lines: &[String]) -> Result<(), &'static str> {
    let mut n = 0;
    let draw = |x| {
        let pixel = if n >= x && n <= x + 2 { '#' } else { '.' };
        print!("{}", pixel);
        n += 1;
        if n >= 40 {        // next line
            println!();
            n = 0;
        }
    };
    process(lines, draw)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
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
        assert_eq!(Ok(13140), task1(data));
    }
}
