use std::io;

fn main() {

    let mut numbers_string = String::new();
    io::stdin().read_line(&mut numbers_string);

    let mut resultat = "".to_string();
    
    for _i in 0..(numbers_string.len() - 2) *2 {
        resultat += &"e".to_string();
    }

    print!("h{}y", resultat);
  
}