// https://open.kattis.com/problems/knightpacking
// Rust
// Easy

use std::io::stdin;

fn main() {
    let mut input = String::new();
    println!("masukan variable: ");
    stdin().read_line(&mut input).unwrap();

    println!("{}", decide(input.trim().parse::<i32>().unwrap()));
}

fn decide(value: i32) -> String {
    if value % 2 == 0 {
        return "second".to_string();
    }
    return "first".to_string();
}

#[cfg(test)]
mod samples {
    use crate::decide;

    #[test]
    fn sample_one() {
        let val = 1;
        let result = decide(val);
        let expected = "first";

        assert_eq!(result, expected);
    }

    #[test]
    fn sample_two() {
        let val = 2;
        let result = decide(val);
        let expected = "second";

        assert_eq!(result, expected);
    }

    #[test]
    fn sample_three() {
        let val = 3;
        let result = decide(val);
        let expected = "first";

        assert_eq!(result, expected);
    }
}