use std::io::{BufRead, self, BufWriter, Write};
use std::collections::VecDeque;

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut buffer = String::new();

    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    reader.read_line(&mut buffer).unwrap();
    let input1: usize = buffer.trim().parse().unwrap();

    let mut deque = VecDeque::new();

    for _ in 0..input1 {
        buffer.clear();
        reader.read_line(&mut buffer).unwrap();
        let input2: Vec<usize> = buffer.split_whitespace().map(|s| s.parse().unwrap()).collect();

        match input2[0] {
            1 => {
                deque.push_front(input2[1])
            },
            2 => {
                deque.push_back(input2[1])
            },
            3 => {
                if let Some(num) = deque.pop_front() {
                    writeln!(writer, "{}", num).unwrap();
                } else {
                    writeln!(writer, "-1").unwrap();
                }
            },
            4 => {
                if let Some(num) = deque.pop_back() {
                    writeln!(writer, "{}", num).unwrap();
                } else {
                    writeln!(writer, "-1").unwrap();
                }
            },
            5 => {
                writeln!(writer, "{}", deque.len()).unwrap()
            },
            6 => {
                if deque.is_empty() {
                    writeln!(writer, "1").unwrap();
                } else {
                    writeln!(writer, "0").unwrap();
                }
            },
            7 => {
                if let Some(num) = deque.front() {
                    writeln!(writer, "{}", num).unwrap();
                } else {
                    writeln!(writer, "-1").unwrap();
                }
            },
            8 => {
                if let Some(num) = deque.back() {
                    writeln!(writer, "{}", num).unwrap();
                } else {
                    writeln!(writer, "-1").unwrap();
                }
            },
            _ => {}
        }
    }
}