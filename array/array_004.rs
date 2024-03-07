/* Programmer:          Per Stoor
 * Date:                2024-03-07
 * Last changed:        2024-03-07
 * Type of program:     enter an array manually and copy into another array (or vector)
 */

use std::io::{self, Write};

fn main(){ 

    print!("Enter Vector size: ");
    let vector_size = match string_to_int() {
        Ok(valid_size)  => valid_size,
        Err(err)        => println!("{}", err),
    };
        if vector_size > 0 {

            //TODO
            // Finish this block of code.
            let original_vector: Vec<i32> = Vec::new();

        }
        else {
            println!("Size of Vector too small!");
            println!("Ending program!");
        }

} 

fn string_to_int() -> Result<i32, String> {

    let mut string_to_int_buffer = String::new();

    io::stdout()
        .flush()
        .unwrap();

    io::stdin()
        .read_line(&mut string_to_int_buffer)
        .expect("Error: Failed to read string!");

    let string_to_int_buffer: i32 = match string_to_int_buffer
        .trim()
        .parse() {
            Ok(parsed_value)    => parsed_value,
            Err(err)              => return Err(String::from("Error: you did not enter a number!")),
    };

return Ok(parsed_value);
}
