/* Programmer:          Per Stoor
 * Date:                2023-04-30
 * Last changed:        2023-04-30
 * Type of program:     Finding the sum of all vector values.
 */

use std::io;
use std::io::Write;

fn main(){ 

    introduction();
    print!("Input the number of elements to be stored in the vector: ");
    let user_inputted_vector_length = string_to_int_user_input();
    let mut vector1: Vec<u32> = Vec::new();

        if user_inputted_vector_length == 1{
            println!("Enter {} element: ", user_inputted_vector_length);
        }
        else{
            println!("Enter {} elements: ", user_inputted_vector_length);
        }
            for loop_counter1 in 0..user_inputted_vector_length{
                print!("Element - {}: ", loop_counter1);
                let vector_element_value = string_to_int_user_input();
                vector1.push(vector_element_value);
            }

    let vector_sum: u32 = vector1
        .iter()
        .sum();

    println!("The vector: {:?}", vector1);
    println!("The total sum = {}", vector_sum);
} 

fn introduction(){
    println!("----------------------------------------");
    println!("Since this is Rust and not C. We will be");
    println!("using Vectors instead of Arrays.");
    println!("----------------------------------------");
}

fn string_to_int_user_input() -> u32{
    let mut function_user_input = String::new();
    io::stdout()
        .flush()
        .unwrap();
    io::stdin()
        .read_line(&mut function_user_input)
        .expect("Not a string...");
    let function_user_input: u32 = function_user_input
        .trim()
        .parse()
        .expect("Not a number...");

return function_user_input;
}
