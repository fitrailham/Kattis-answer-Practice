use std::{self, io};

fn main() {

        let mut word = String::new();
        io::stdin().read_line(&mut word);
        let res = word.replace("\n","");
       println!("{} {} {}", res, res, res);
        // suck_in_a_time_loop


}