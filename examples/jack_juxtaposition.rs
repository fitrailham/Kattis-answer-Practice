use std::{self, io};

fn main() {

    let mut user_input = String::new();
    let stdin = io::stdin(); 
    stdin.read_line(&mut user_input);
    let v: Vec<_> = user_input.split(|c| c == ',' || c == ' ').collect();
    let a = v[0].trim().parse::<i32>().unwrap();
    let b = v[1].trim().parse::<i32>().unwrap();
    let c = v[2].trim().parse::<i32>().unwrap();
    let res = (a*b)*c;

    println!("{:?}", res);
}