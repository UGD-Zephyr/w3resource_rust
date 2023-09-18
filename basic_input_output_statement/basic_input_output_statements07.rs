/* Programmer:          Per Stoor
 * Date:                2023-09-18
 * Last changed:        2023-09-18
 * Type of program:     Reading first name, last name and date of birth
 *                      and displaying them sequentially.
 */

use std::io;
use std::io::Write;
use std::process::Command;

fn main(){ 

    clear_screen();

    print!("Enter first name: ");
    let first_name = string_reader();
    print!("Enter last name: ");
    let last_name = string_reader();
    print!("Enter date of birth: ");
    let date_of_birth = string_to_integer();

    println!("{} {} {}", first_name, last_name, date_of_birth);

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
        .expect("Error: Cannot read string.");

return string_buffer.trim().to_string();

}

fn string_to_integer() -> u32 {
    let mut string_integer_buffer = String::new();

    io::stdout()
        .flush()
        .unwrap();

    io::stdin()
        .read_line(&mut string_integer_buffer)
        .expect("Error: Cannot read string.");

    let string_integer_buffer: u32 = string_integer_buffer
        .trim()
        .parse()
        .expect("Error: Cannot convert string to integer.");

return string_integer_buffer;
}
