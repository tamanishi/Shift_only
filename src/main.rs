use std::io;

fn main() {
    let mut line1 = String::new();
    let _ = io::stdin().read_line(&mut line1);
    let mut line2 = String::new();
    let _ = io::stdin().read_line(&mut line2);
    let mut values = line2
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|e| e.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut cnt = 0;

    loop {
        match values.iter().any(|e| (e % 2) == 1) {
            true => break,
            false => cnt = cnt + 1,
        }
        values = values.iter().map(|e| e / 2).collect::<Vec<i32>>();
    }

    println!("{}", cnt);
}
