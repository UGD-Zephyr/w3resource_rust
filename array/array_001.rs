/* Programmer:          Per Stoor
 * Date:                2023-04-30
 * Last changed:        2023-04-30
 * Type of program:     Store elements in an array and print it.
 */

use std::io;
use std::io::Write;

fn main(){ 

    let mut array1 = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    introduction();

        for loop_counter1 in 0..10{

            print!("Element - {}: ", loop_counter1);
            let mut user_input = String::new();
            io::stdout()
                .flush()
                .unwrap();
            io::stdin()
                .read_line(&mut user_input)
                .expect("Not a string...");

            let user_input: u32 = user_input
                .trim()
                .parse()
                .expect("Not a number...");

            array1[loop_counter1] = user_input;
        }

        print!("Elements in the array are: ");
        for loop_counter1 in 0..10{
            print!(" {}", array1[loop_counter1]);
        }
        println!();


} 

fn introduction(){
    println!("Input 10 elements into the Array: ");
}
