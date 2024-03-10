/* Programmer:          Per Stoor
 * Date:                2024-03-10
 * Last changed:        2024-03-10
 * Type of program:     Counting duplicate vector values.
 */

use std::io::{self, Write};

fn main(){ 

        print!("Please enter vector size: ");
        let vector_size = string_to_int();

            if vector_size > 0 {
                let mut user_vector: Vec<i32> = Vec::new();
                    for loop_counter1 in 0..vector_size {
                        print!("Please enter the {} element: ",loop_counter1);
                        let vector_value = string_to_int();
                            user_vector.push(vector_value);

                    }

                    println!("Vector: {:?}", user_vector);
                    locate_duplicate_vector_values(user_vector);

                }
                else {

                    println!("Vector size needs to be larger than zero!");

                }

} 

fn string_to_int() -> i32 {

    let mut user_string_buffer = String::new();

        io::stdout()
            .flush()
            .expect("Error: Failed flushing stdout!");

        let user_string_buffer = loop {
            user_string_buffer.clear();

            io::stdin()
                .read_line(&mut user_string_buffer)
                .expect("Error: Failed to read string from stdin!");

            match user_string_buffer
                .trim()
                .parse::<i32>() {
                    Ok(parse_success)   => break parse_success,
                    Err(_)              => println!("Error: Enter a number!"), // this NEEDS to be println! Weird errors if it is only print!
            };

        };

return user_string_buffer;
}

fn locate_duplicate_vector_values(function_vector: Vec<i32>) {

}
