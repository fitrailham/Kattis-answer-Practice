// use core::num::dec2flt::number::Number;
// use std::io::Write;
use std::{self, io};
// use std::io::{self, BufRead};


// #[tokio::main]
fn main()-> Result<(), io::Error> {
    // let mut user_input_2 = String::new();

    // let mut user_input = String::new();
    // println!("masukan variable pertama: ");
    // let stdin = io::stdin(); 
    // stdin.read_line(&mut user_input).unwrap();

    // let v: Vec<_> = user_input.split(|c| c == ',' || c == ' ').collect();
    // let a = v[0].trim().parse::<i32>().unwrap();
    // let b = v[1].trim().parse::<i32>().unwrap();
    // // println!("input {:?} === {:?}", a, b);

    // let res =    if a < b {
    //     0
    // } else if a == b{
    //     0
    // }else {
    //     1
    // };

    // println!("test => {:?}", res);
    // let mut word = String::new();
    // println!("masukan variable: ");
    // io::stdin().read_line(&mut word)?;

    // let mut container = Vec::new();
    
    // for _i in 0..3{
    //     container.push(word.clone())
    // }

    // let res = (container.join(" ")).replace('\n', "");

    // println!("res => {:?}", res);


// fn main() {
    // let stdin = io::stdin();
    // for line in stdin.lock().lines().map(|l| l.unwrap()) {
    //     let nums: Vec<i64> = line.split_whitespace()
    //         .map(|num| num.parse().unwrap())
    //         .collect();
    //     let a = nums[0];
    //     let b = nums[1];
    //     println!("{}", (a - b).abs());
    // }

    
// }

    Ok(())

    // println!("masukan variable kedua: ");
    // let stdin_2 = io::stdin(); // We get `Stdin` here.
    // stdin_2.read_line(&mut user_input_2);

    // let s = "1,2 3";
    // let v: Vec<_> = user_input.split([' '].as_ref()).collect();
    // assert_eq!(v, ["1", "2", "3"]);
    

    // println!("res => {}", res);

    // for i in 0..v.len() {
    //     // let s = i.replace(&['(', ')', ',', '\"', '.', ';', ':', '\''][..], "");
    //     print!("res => {:?}", s)
    // }

    // for i in := 0; i < 3; i++ {
    //     fmt.Println(from, ":", i)
    // }

    // for loop_variable in v {
    //     println!("test data {}", loop_variable)
    // }

    // let a = user_input.trim().parse::<i32>().unwrap();
    // let b = user_input_2.trim().parse::<i32>().unwrap();

    // // let res = comparation_number(one, two);
    // // println!("res => {}", res);
    // let res;

    // if a < b {
    //     res = 0;
    // } else if a == b{
    //     res = 0;
    // }else {
    //     res = 1;
    // }

    // println!("res => {}", res);

    // let stdin = io::stdin();
    // for line in stdin.lock().lines().map(|l| l.unwrap()) {
    //     let nums: Vec<i64> = line.split_whitespace()
    //         .map(|num| num.parse().unwrap())
    //         .collect();
    //     let a = nums[0];
    //     let b = nums[1];
    // }
  
}

// fn comparation_number(a: i32, b: i32) -> i32{
//     let res;

//     if a < b {
//         res = 0;
//     } else if a == b{
//         res = 0;
//     }else {
//         res = 1;
//     }
//     return res;
//   }