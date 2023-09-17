/* Programmer:          Per Stoor
 * Date:                2023-09-17
 * Last changed:        2023-09-17
 * Type of program:     KM/H <-> MP/H converter.
 */

use std::io;
use std::io::Write;
use std::process::Command;
const KILOMETER_CONSTANT: f32 = 0.62137119;
const MILE_CONSTANT: f32 = 1.609344;

fn main(){ 

    let mut user_choice = 0.0;
    while user_choice != 3.0{

        conversion_menu();
        user_choice = string_to_float();
            if user_choice == 1.0{

                

                print!("Please enter Kilometer per hour: ");
                let kilometer = string_to_float();
                let mile = kilometer_to_mile(kilometer);

                println!("{:.2} Kilometer / Hour equals {:.2} Miles / Hour", kilometer, mile);
                print!("Press Enter to start over.");
                let _ = string_to_empty_line();
                clear_screen(); 
            }
            else if user_choice == 2.0 {

                print!("Please enter Miles per hour: ");
                let mile = string_to_float();
                let kilometer = mile_to_kilometer(mile);

                println!("{:.2} Miles / Hour equal {:.2} Kilometers / Hour", mile, kilometer);
                print!("Press Enter to start over.");
                let _ = string_to_empty_line();
                clear_screen(); 
            }
            else if user_choice == 3.0 {

                println!("Exiting program...");
            }
            else{
                println!("Error: Menu selection failed.");
                print!("Press Enter to start over.");
                let _ = string_to_empty_line();
                clear_screen(); 
            }

    }

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

fn conversion_menu(){

    println!("1. Kilometer to Mile.");
    println!("2. Mile to Kilometer .");
    println!("3. Exit program.");
}

fn string_to_empty_line(){
    
    let mut string_empty_buffer = String::new();

    io::stdout()
        .flush()
        .unwrap();

    io::stdin()
        .read_line(&mut string_empty_buffer)
        .expect("Error: Cannot read string input.");

}

fn string_to_float() -> f32{

    let mut string_float_buffer = String::new();

    io::stdout()
        .flush()
        .unwrap();

    io::stdin()
        .read_line(&mut string_float_buffer)
        .expect("Error: Cannot read string input.");

    let string_float_buffer: f32 = string_float_buffer
        .trim()
        .parse()
        .expect("Error: cannot convert string to f32 float.");

return string_float_buffer;
}

fn kilometer_to_mile(kilometer_value: f32) -> f32{

    let mile_value = kilometer_value * KILOMETER_CONSTANT;

return mile_value;
}

fn mile_to_kilometer(mile_value: f32) -> f32{

    let kilometer_value = mile_value * MILE_CONSTANT;

return kilometer_value;
}
