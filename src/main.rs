use std::io::{BufRead, self};

fn read_line() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().lock().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

fn main() {
    let input = read_line().unwrap();
    let char_vec = input.chars().map(|s| s.to_ascii_lowercase()).collect::<Vec<_>>();
    let mut kind = char_vec.clone();
    kind.sort();
    kind.dedup();

    let mut t1 = ' ';
    let mut c1 = 0;
    let mut c2 = 0;
    for s in kind {
        let count = char_vec.iter().filter(|&t| *t==s).count();
        if count > c1 {
            c1 = count;
            t1 = s;
        } else if count == c1 {
            c2 = count;
        }
    }

    if c1 == c2 {
        println!("?")
    } else {
        println!("{}", t1.to_ascii_uppercase())
    }
}
