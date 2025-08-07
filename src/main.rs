use std::io::{BufRead, self};

fn read_line() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().lock().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}
fn main() {
    let _ = read_line();
    let num_vec = read_line().unwrap().split_whitespace().map(|s| s.parse::<i16>().unwrap()).collect::<Vec<_>>();
    let max = num_vec.iter().max().unwrap();
    let mut sum = 0f32;

    for i in &num_vec {
        sum += (*i as f32)/(*max as f32)*100f32;
    }
    println!("{}", sum/(num_vec.len() as f32));
}
