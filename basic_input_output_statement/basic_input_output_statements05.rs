/* Programmer:          Per Stoor
 * Date:                2023-09-17
 * Last changed:        2023-09-17
 * Type of program:     Calculate total minutes from user inputted
 *                      hours and minutes.
 */

use std::io;
use std::io::Write;
use std::process::Command;

fn main(){ 

    clear_screen();

    print!("Enter hours: ");
    let hours = string_to_integer();
    print!("Enter minutes: ");
    let minutes = string_to_integer();

    let total_minutes = time_compression(hours, minutes);
    println!("Total minutes = {}", total_minutes);

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

fn string_to_integer() -> u32{

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
        .expect("Error: Cannot parse string to integer.");

return string_integer_buffer;
}

fn time_compression(hours_value: u32, minutes_value: u32) -> u32{

    let total_minutes_value = minutes_value + (hours_value * 60);

return total_minutes_value;
}
