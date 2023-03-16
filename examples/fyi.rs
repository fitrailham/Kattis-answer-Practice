use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let res = if input[..3] == "555".to_string(){
        1
    } else{
        0
    };

    println!("{}", res);
}