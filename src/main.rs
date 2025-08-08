use std::io::{BufRead, self};

fn read_line() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().lock().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}
fn main() {
    let input = read_line().unwrap().chars().collect::<Vec<_>>();

    for i in 'a'..='y' {
        let res = input.iter().position(|&s| s == i);
        match res {
            Some(c) => print!("{} ", c),
            None => print!("-1 ")
        }
    }
    let z = input.iter().position(|&s| s== 'z');
    match z {
        Some(c) => print!("{}", c),
        None => print!("-1")
    }

}
