use std::io::{BufRead, self};

fn read_line() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().lock().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

fn main() {
    let input = read_line().unwrap().parse::<u32>().unwrap();

    for i in 1..input {
        let split_digit = i.to_string().chars().map(|s| s.to_digit(10).unwrap()).collect::<Vec<_>>();
        let res = i + split_digit.iter().sum::<u32>();
        if input == res {
            println!("{}", i);
            return;
        }
    }
    println!("0")
}