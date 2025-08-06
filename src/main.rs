use std::io::{BufRead, self};

fn read_line() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().lock().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}
fn main() {
    let setting = read_line().unwrap().split_whitespace().map(|s| s.parse::<i16>().unwrap()).collect::<Vec<_>>();
    let mut num_vec = vec![0;setting[0] as usize];

    for _i in 0..setting[1] {
        let method = read_line().unwrap().split_whitespace().map(|s| s.parse::<i16>().unwrap()).collect::<Vec<_>>();
        for j in method[0]-1..=method[1]-1 {
            num_vec[j as usize] = method[2];
        }
    }

    println!("{}", num_vec.iter().map(|&s| s.to_string()).collect::<Vec<String>>().join(" "))
}
