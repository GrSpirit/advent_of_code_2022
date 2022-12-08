mod input;
use input::{read_file, read_stdin};
use std::io;
use std::env;

enum Mode {
    File(String),
    Stdin
}

fn task1(lines: &[String]) -> Result<u32, &'static str> {
    let grid = lines.iter().map(|s| s.as_bytes()).collect::<Vec<_>>();
    let n = grid.len();
    let m = grid[0].len();
    let mut visible = vec![vec![0u32; m]; n];

    {
        let mut line = vec![0; m];
        for i in 0..m {
            line[i] = grid[0][i];
            visible[0][i] = 1;
        }
        for i in 1..n-1 {
            for j in 1..m-1 {
                if line[j] < grid[i][j] {
                    line[j] = grid[i][j];
                    visible[i][j] = 1;
                }
            }
        }
    }
    {
        let mut line = vec![0; m];
        for i in 0..m {
            line[i] = grid[n-1][i];
            visible[n-1][i] = 1;
        }
        for i in (1..n-1).rev() {
            for j in 1..m-1 {
                if line[j] < grid[i][j] {
                    line[j] = grid[i][j];
                    visible[i][j] = 1;
                }
            }
        }
    }
    {
        let mut line = vec![0; n];
        for i in 0..n {
            line[i] = grid[i][0];
            visible[i][0] = 1;
        }
        for j in 1..m-1 {
            for i in 1..n-1 {
                if line[i] < grid[i][j] {
                    line[i] = grid[i][j];
                    visible[i][j] = 1;
                }
            }
        }
    }
    {
        let mut line = vec![0; n];
        for i in 0..n {
            line[i] = grid[i][m-1];
            visible[i][m-1] = 1;
        }
        for j in (1..m-1).rev() {
            for i in 1..n-1 {
                if line[i] < grid[i][j] {
                    line[i] = grid[i][j];
                    visible[i][j] = 1;
                }
            }
        }
    }

    Ok(visible.iter().fold(0, |acc, row| acc + row.iter().sum::<u32>()))
}

fn task2(lines: &[String]) -> Result<u32, &'static str> {
    fn vision_rate(grid: &Vec<&[u8]>, ii: usize, jj: usize) -> u32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut view = vec![0; 4];
        for i in (0..ii).rev() {
            view[0] += 1;
            if grid[i][jj] >= grid[ii][jj] {
                break;
            }
        }
        for i in ii+1..n {
            view[1] += 1;
            if grid[i][jj] >= grid[ii][jj] {
                break;
            }
        }
        for j in (0..jj).rev() {
            view[2] += 1;
            if grid[ii][j] >= grid[ii][jj] {
                break;
            }
        }
        for j in jj+1..m {
            view[3] += 1;
            if grid[ii][j] >= grid[ii][jj] {
                break;
            }
        }
        view.into_iter().product()
    }
    let grid = lines.iter().map(|s| s.as_bytes()).collect::<Vec<_>>();
    let n = grid.len();
    let m = grid[0].len();
    let mut best_rate = 0;
    for i in 1..n-1 {
        for j in 1..m-1 {
            best_rate = best_rate.max(vision_rate(&grid, i, j));
        }
    }
    Ok(best_rate)
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
    match task2(&data) {
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
        assert_eq!(Ok(21), task1(&data));
    }
    #[test]
    fn test2() {
        let data = &[
            "30373",
            "25512",
            "65332",
            "33549",
            "35390",
        ].into_iter().map(|s| s.to_string()).collect::<Vec<String>>();
        assert_eq!(Ok(8), task2(&data));
    }
}
