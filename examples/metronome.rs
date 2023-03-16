use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    println!("{:.2}", input.trim().parse::<f64>().unwrap() / 4.0);
}