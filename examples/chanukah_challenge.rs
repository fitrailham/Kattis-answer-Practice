// # Time: 2022-08-13 09:56:42
// # title: Chanukah Challenge
// # language: Python 3


// t = int(input())
// for i in range(t):
//     k, n = map(int, input().split())
//     a = n * (n+3)//2
//     print(k, a)

use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    // println!("{:.2}", input.trim().parse::<f64>().unwrap() / 4.0);
    // let a = input.trim().parse::<i32>().unwrap();
    let numbers = input.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    println!("number => {:?}", numbers);

    // if a % 2 == 0 {
    //     println!("Bob")
    // }else {
    //     println!("Alice")
    // };


}