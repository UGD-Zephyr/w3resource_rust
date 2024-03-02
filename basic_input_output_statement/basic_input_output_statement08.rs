/* Programmer:          Per Stoor
 * Date:                2024-01-14
 * Last changed:        2024-01-14
 * Type of program:     Calculate the sum of three numbers inputted
 *                      as one line separated with a comma.
 */

use std::io;
use std::io::Write;
use std::process::Command;

fn main(){ 

    clear_screen();
    print!("Enter numbers separated with a comma: ");
    let user_input = string_reader();
    println!("This is the result of the function string_reader: {}", user_input);

} 

fn clear_screen(){
    if cfg!(unix){
        let _ = Command::new("clear")
            .status();
    }
    else if cfg!(windows){
        let _ = Command::new("cmd")
            .arg("/c")
            .arg("cls")
            .status();
    }
}

fn string_reader() -> String {
    let mut string_buffer = String::new();

    io::stdout()
        .flush()
        .unwrap();
    
    io::stdin()
        .read_line(&mut string_buffer)
        .expect("Error: Cannot read string");

return string_buffer.trim()
        .to_string();
}

fn string_to_integer() -> u32 {
    let mut string_to_integer_buffer = String::new();

    io::stdout()
        .flush()
        .unwrap();

    io::stdin()
        .read_line(&mut string_to_integer_buffer)
        .expect("Error: Cannot read string.");

    let string_to_integer_buffer: u32 = string_to_integer_buffer
        .trim()
        .parse()
        .expect("Error: Cannot convert string to integer");

return string_to_integer_buffer;
}
