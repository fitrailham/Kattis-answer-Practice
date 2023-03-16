use std::io;

fn main() {

    let mut numbers_string = String::new();
    io::stdin().read_line(&mut numbers_string);

    /*#region Processing */
    // let numbers = numbers_string.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>(); //pake ini biar gampang nikin string to angka
    let numbers = numbers_string.trim().parse::<i32>().unwrap();
    
    for i in 0..numbers {
        println!("{} Abracadabra", i+1);
    }
  
}