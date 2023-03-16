

use std::io;

fn main() {

    let mut numbers_string = String::new();
    io::stdin().read_line(&mut numbers_string);

    /*#region Processing */
    let numbers = numbers_string.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>(); //pake ini biar gampang nikin string to angka
    let min_value = *numbers.iter().max().unwrap();
    let mut output = "".to_string();
    
    for i in 1..min_value+1 {
        if i % numbers[0] == 0 && i % numbers[1] == 0{
            println!("FizzBuzz");
        } else if i % numbers[0] == 0 {
            println!("Fizz");
            
        } else if i % numbers[1] == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}