use std::{self, io};

fn main() {

    let mut user_input = String::new();
    // println!("masukan variable pertama: ");
    let stdin = io::stdin(); 
    stdin.read_line(&mut user_input);

    let v: Vec<_> = user_input.split(|c| c == ',' || c == ' ').collect();
    let a = v[0].trim().parse::<i32>().unwrap();
    let b = v[1].trim().parse::<i32>().unwrap();

    let res =    if a < b {
        0
    } else if a == b{
        0
    }else {
        1
    };

    println!("{:?}", res);
}