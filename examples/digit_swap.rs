/* The input consists of a single 2-digit code with only digits to , without any space between the digits. */

use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    // println!("{:.2}", input.trim().parse::<f64>().unwrap() / 4.0);
    // let a = input.trim().parse::<i32>().unwrap();
    let res = input.chars().rev().collect::<String>();
    println!("{}", res);
   

}