/* Programmer:          Per Stoor
 * Date:                2023-09-15
 * Last changed:        2023-09-15
 * Type of program:     Converting Celsius to Fahrenheit and vice versa.
 */

use std::io;
use std::io::Write;

fn main(){ 

    introduction();
    menu();
    let user_choice = string_to_integer();
        if user_choice == 1{

            println!("Enter degrees in Celsius:");
            let celsius = string_to_float();
            celsius_to_fahrenheit_converter(celsius);
        }
        else if user_choice == 2{

            println!("Enter degrees in Fahrenheit:");
            let fahrenheit = string_to_float();
            fahrenheit_to_celsius_converter(fahrenheit);
        }
        else{

            println!("1 or 2 was not selected. Run the program again please.");
        }

} 

fn introduction(){

    println!("This program will convert between Celius and Fahrenheit.");
}
fn menu(){

    println!("1. Celius to Fahrenheit");
    println!("2. Fahrenheit to Celius ");
}
fn string_to_integer() -> u32{
    
   let mut string_integer_buffer = String::new();
   io::stdout()
       .flush()
       .unwrap();
   io::stdin()
       .read_line(&mut string_integer_buffer)
       .expect("Error: Can't convert to integer.");

   let string_integer_buffer: u32 = string_integer_buffer
       .trim()
       .parse()
       .expect("Error: Not u32 integer");

return string_integer_buffer;
}
fn string_to_float() -> f32{

   let mut string_float_buffer = String::new();
   io::stdout()
       .flush()
       .unwrap();
   io::stdin()
       .read_line(&mut string_float_buffer)
       .expect("Error: Can't convert to float.");

   let string_float_buffer: f32 = string_float_buffer
       .trim()
       .parse()
       .expect("Error: Not f32 float");

return string_float_buffer;
}
fn celsius_to_fahrenheit_converter(celsius_value: f32){

    let converted_fahrenheit = (celsius_value * (9.0/5.0)) + 32.0;
    println!("{} Celsius = {} Fahrenheit.", celsius_value, converted_fahrenheit);
}
fn fahrenheit_to_celsius_converter(fahrenheit_value: f32){

    let converted_celsius = (fahrenheit_value - 32.0) * (5.0/9.0);
    println!("{} Fahrenheit = {} Celsius.", fahrenheit_value, converted_celsius);
}
