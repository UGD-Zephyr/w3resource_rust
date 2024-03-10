/* Programmer:          Per Stoor
 * Date:                2024-03-07
 * Last changed:        2024-03-09
 * Type of program:     enter an array manually and copy into another array (or vector)
 */

use std::io::{self, Write};

fn main(){ 

    print!("Enter Vector size: ");
    let vector_size = string_to_int();
        if vector_size > 0 {
            let mut original_vector: Vec<i32> = Vec::new();
                for loop_counter1 in 0..vector_size {
                    print!("Enter {} element: ", loop_counter1);
                    let vector_value = string_to_int(); 
                    original_vector.push(vector_value);
                }
                        let cloned_vector = original_vector.clone();    // We use the .clone()
                                                                        // method for simple vector
                                                                        // copying
                        println!("Original vector = {:?}", original_vector);
                        println!("Cloned vector = {:?}", cloned_vector);
        }
        else {
            println!("Size of Vector too small!");
            println!("Ending program!");
        }

} 

fn string_to_int() -> i32 {
    let mut string_to_int_buffer = String::new();
    io::stdout()
        .flush()
        .expect("Error: Failed to flush stdout!");  // unwrap() is ok because this kind of stdout flushing
                                                    // rarely fails and a panic! is acceptable here

    let string_to_int_buffer = loop {
        string_to_int_buffer.clear();
    io::stdin()
        .read_line(&mut string_to_int_buffer)
        .expect("Error: Failed to read string!");   // Same as above, .expect is ok becasue all
                                                    // terminal input should be a string
     match string_to_int_buffer
        .trim()
        .parse::<i32>() {
            Ok(parsed_value)    => break parsed_value,
            Err(_)              => println!("Error: Need numeric input."),
        };
    };
return string_to_int_buffer;
}
