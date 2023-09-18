/* Programmer:          Per Stoor
 * Date:                2023-09-18
 * Last changed:        2023-09-18
 * Type of program:     Program that takes minutes as input and returns
 *                      amount of hours and minutes together.
 */

use std::io;
use std::io::Write;
use std::process::Command;

fn main(){ 

    clear_screen();
    print!("Enter minutes: ");
    let minutes = string_to_integer();
    time_compression(minutes);

} 

fn clear_screen(){
    if cfg!(unix) {
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

fn time_compression(minute_value: u32){
    let hour_value = minute_value / 60;
    let remainder_minutes = minute_value % 60;

    println!("{} Minutes becomes:", minute_value);
    println!("{} Hour(s)", hour_value);
    println!("{} Minute(s)", remainder_minutes);
}
