// Betting
// https://open.kattis.com/problems/betting


use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let a = input.trim().parse::<f64>().unwrap();
    println!("{:?}", 100.00/a);
    print!("{:?}", 100.00/(100.00-a));
}