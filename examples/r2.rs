/*
<summary>
    Author : Htet Aung  Hlaing
</summary>
*/

/*#region Import Area */
use std::io;
/*#endregion */

fn main() {

    //Setting input stream
    let input = io::stdin();

    /*#region reading line */
    let mut numbers_string = String::new();
    input.read_line(&mut numbers_string);
    /*#endregion */

    /*#region Processing */
    let numbers = numbers_string.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let mean = numbers[1];
    let number1 = numbers[0];
    let number2 = (2 * mean) - number1;
    /*#endregion */

    print!("{}", number2);
}