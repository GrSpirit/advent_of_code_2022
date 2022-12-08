use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let mut total = 0;
    let mut current = 0;
    let mut max_weight = 0;
    let mut lines = io::stdin().lock().lines();
    while let Some(line) = lines.next() {
        let input = line?;
        if input.is_empty() {
            current = 0;
        } else {
            let x: i32 = input.parse().unwrap();
            current += x;
            total += x;
            max_weight = max_weight.max(current);
        }
    }
    println!("max of one = {}\ntotal = {}", max_weight, total);
    Ok(())
}
