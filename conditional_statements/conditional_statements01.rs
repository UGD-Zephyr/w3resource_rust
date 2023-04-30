/* Programmer:          Per Stoor
 * Date:                2023-04-30
 * Last changed:        2023-04-30
 * Type of program:     Accept two integers and check if they are equal.
 */

use std::io;
use std::io::Write;

fn main(){ 

    print!("Enter a number: ");
    let user_inputted_number1 = string_to_integer();

    print!("Enter a number: ");
    let user_inputted_number2 = string_to_integer();

    let number_result = compare_equal(user_inputted_number1, user_inputted_number2);
        if number_result == true {
            println!("{} and {} are equal", user_inputted_number1, user_inputted_number2);
        }
        else if number_result == false{
            println!("{} and {} are NOT equal", user_inputted_number1, user_inputted_number2);
        }
        else{
            println!("number_result was not true or false...");
        }

} 

fn string_to_integer() -> u32{

    let mut string_buffer = String::new();

    io::stdout()
        .flush()
        .unwrap();
    io::stdin()
        .read_line(&mut string_buffer)
        .expect("Not a string...");

    let string_buffer: u32 = string_buffer
        .trim()
        .parse()
        .expect("Not a number...");

return string_buffer;
}

fn compare_equal(number1: u32, number2: u32) -> bool{

    if number1 == number2{
        let equal= true;
        return equal;
    }
    else{
        let equal = false;
        return equal;
    }
}
