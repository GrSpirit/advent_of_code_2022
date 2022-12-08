use std::env;
use std::io::{self, BufRead};
use std::fs::File;
use std::str::FromStr;

pub fn read_file(file_name: &str) -> io::Result<Vec<String>> {
    let mut result = Vec::new();
    let file = File::open(file_name)?;
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        result.push(line?);
    }
    Ok(result)
}

#[derive(Debug, Clone, Copy)]
enum Suefa {
    Rock,
    Paper,
    Scissors
}

impl FromStr for Suefa {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Suefa::Rock),
            "B" => Ok(Suefa::Paper),
            "C" => Ok(Self::Scissors),
            _ => Err("cannot parse")
        }
    }
}

fn tool_score(suefa: Suefa) -> u32 {
    match suefa {
        Suefa::Rock => 1,
        Suefa::Paper => 2,
        Suefa::Scissors => 3
    }
}

#[derive(Debug, Clone, Copy)]
enum FightResult {
    Lose,
    Draw,
    Win
}

fn fight(a: Suefa, b: Suefa) -> FightResult {
    use Suefa::*;
    use FightResult::*;
    match (a, b) {
        (Rock, Rock) => Draw,
        (Rock, Paper) => Lose,
        (Rock, Scissors) => Win,
        (Paper, Rock) => Win,
        (Paper, Paper) => Draw,
        (Paper, Scissors) => Lose,
        (Scissors, Rock) => Lose,
        (Scissors, Paper) => Win,
        (Scissors, Scissors) => Draw
    }
}

fn fight_score(fight_result: FightResult) -> u32 {
    match fight_result {
        FightResult::Lose => 0,
        FightResult::Draw => 3,
        FightResult::Win => 6
    }
}

fn calc_score(games: &[String]) -> u32 {
    let mut total = 0;
    for game in games {
        let g = game.split_ascii_whitespace().collect::<Vec<_>>();
        let (player1, player2) = (g[0], g[1]);
        let tool2 = match player2 {
            "X" => Suefa::Rock,
            "Y" => Suefa::Paper,
            "Z" => Suefa::Scissors,
            _ => unreachable!()
        };
        let tool1: Suefa = player1.parse().unwrap();
        let score = tool_score(tool2) + fight_score(fight(tool2, tool1));
        total += score;
    }
    total
}

fn win_startegy(games: &[String]) -> u32 {
    let mut total = 0;
    for game in games {
        let g = game.split_ascii_whitespace().collect::<Vec<_>>();
        let (player1, player2) = (g[0], g[1]);
        let mut score = match player2 {
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => unreachable!()
        };

        score += match (player2, player1) {
            ("X", "A") => 3,
            ("X", "B") => 1,
            ("X", "C") => 2,
            ("Y", "A") => 1,
            ("Y", "B") => 2,
            ("Y", "C") => 3,
            ("Z", "A") => 2,
            ("Z", "B") => 3,
            ("Z", "C") => 1,
            _ => unreachable!()
        };
        total += score;
    }
    total
}

fn main() -> io::Result<()>{
    let file_name = env::args().nth(1).unwrap_or_default();
    if file_name.is_empty() {
        panic!("No file name");
    }
    let lines = read_file(&file_name)?;
    println!("{}", calc_score(&lines));
    println!("{}", win_startegy(&lines));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let data = vec!["A Y".to_owned(), "B X".to_owned(), "C Z".to_owned()];
        assert_eq!(calc_score(&data), 15);
        assert_eq!(win_startegy(&data), 12);
    }
}
