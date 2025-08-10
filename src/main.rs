use std::io::{BufRead, self};
use std::collections::HashMap;

fn read_line() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().lock().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

fn main() {
    let dict: HashMap<&str, f32> = [("A+", 4.5), ("A0", 4.0), ("B+", 3.5), ("B0", 3.0), ("C+", 2.5), ("C0", 2.0), ("D+", 1.5), ("D0", 1.0), ("F", 0.0)]
        .iter().cloned().collect();
    let mut divisor = 0.0; // 학점 * 과목평점(dict)의 합
    let mut dividend = 0.0; // 학점의 합

    for _i in 0..20 {
        let input =  read_line().unwrap();
        let input_vec = input.split_whitespace().collect::<Vec<_>>();

        if input_vec[2] == "P" {
            continue
        } else {
            let unit = input_vec[1].parse::<f32>().unwrap();
            let subject_point = *dict.get(input_vec[2]).unwrap();
            divisor += unit * subject_point;
            dividend += unit;
        }
    }

    println!("{}", divisor/dividend);
}
