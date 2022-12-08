mod input;
mod priority_queue;

use std::env;
use std::io;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use priority_queue::PriorityQueue;

enum Mode {
    Stdin,
    File(String),
}

fn process(input_stream: Receiver<String>, result_sender: Sender<Vec<i32>>) {
    let mut current = 0;
    let mut top = PriorityQueue::new();
    let mut update_top = |x| {
        top.push(x);
        if top.len() > 3 {
            top.pop();
        }
    };
    while let Ok(input) = input_stream.recv() {
        if input.is_empty() {
            update_top(current);
            current = 0;
        } else {
            let x: i32 = input.parse().unwrap();
            current += x;
        }
    }
    update_top(current);
    result_sender.send(top.into_iter().collect()).unwrap();
}

fn main() -> Result<(), io::Error> {
    let mode = if let Some(file_name) = env::args().nth(1) {
        Mode::File(file_name)
    } else {
        Mode::Stdin
    };
    let (sender, receiver) = channel();
    let (result_sender, result_receiver) = channel();
    let handle = thread::spawn(move || process(receiver, result_sender));

    match mode {
        Mode::Stdin => input::read_stdin(sender)?,
        Mode::File(file_name) => input::read_file(sender, file_name)?
    }

    let top = result_receiver.recv().unwrap();
    handle.join().unwrap();
    println!("{} {} {}", top[0], top[1], top[2]);
    println!("total {}", top.into_iter().sum::<i32>());
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc::channel;
    use std::thread;
    use super::*;
    #[test]
    fn top3() {
        let test_data = vec!["1", "2", "", "3", "4", "", "5", "6", "", "7", "", "1", "", "9"];
        let (sender, receiver) = channel();
        let (result_sender, result_receiver) = channel();
        let handle = thread::spawn(move || process(receiver, result_sender));
        for s in test_data {
            sender.send(s.to_owned()).unwrap();
        }
        drop(sender);
        let top = result_receiver.recv().unwrap();
        handle.join().unwrap();
        assert_eq!(vec![11, 9, 7], top);
    }
}
