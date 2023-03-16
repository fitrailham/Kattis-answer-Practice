// GCVWR
// https://open.kattis.com/problems/gcvwr

use std::io;

fn main() {
    println!("test aja");
    let mut line1 = String::new();
    io::stdin().read_line(&mut line1).expect("Failed to read line");
    let numbers1 = line1.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let mut line2 = String::new();
    io::stdin().read_line(&mut line2).expect("Failed to read line");
    let numbers2 = line2.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let mut sum_line_2 = 0;
    for i in numbers2{
        sum_line_2 += i
    }

    let total = numbers1[0] - numbers1[1];
    println!("{:?}",(total as f64 * 0.9) as i32 - sum_line_2);
}
