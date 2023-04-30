/* Programmer:          Per Stoor
 * Date:                2023-04-30
 * Last changed:        2023-04-30
 * Type of program:     Make a dynamic array and print it in reverse.
 *
 * Note:                In rust, this will be done with Vectors.
 */

use std::io;
use std::io::Write;

fn main(){ 

    introduction();

    let mut vector1: Vec<u32> = Vec::new();

    print!("Input the number of elements to store in the vector: ");
    let mut user_input_vector_length = String::new();
        io::stdout()
            .flush()
            .unwrap();
        io::stdin()
            .read_line(&mut user_input_vector_length)
            .expect("Not a string...");

    let user_input_vector_length: u32 = user_input_vector_length
        .trim()
        .parse()
        .expect("Not a number...");
 
        for loop_counter1 in 0..user_input_vector_length{
            print!("Element - {}: ", loop_counter1);
            let mut user_input_vector_value = String::new();
            io::stdout()
                .flush()
                .unwrap();
            io::stdin()
                .read_line(&mut user_input_vector_value)
                .expect("Not a string...");

            let user_input_vector_value: u32 = user_input_vector_value
                .trim()
                .parse()
                .expect("Not a number...");

            vector1.push(user_input_vector_value);
        }

        println!("The last vector: {:?}", vector1);


} 

fn introduction(){
    println!("-------------------------------------------");
    println!("Since this is Rust and not C.");
    println!("We will be using Vectors instead of Arrays.");
    println!("-------------------------------------------");
}
