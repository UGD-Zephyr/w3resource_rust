/* Programmer:          Per Stoor
 * Date:                2023-09-17
 * Last changed:        2023-09-17
 * Type of program:     Rust program to calculate the perimeter of a rectangle.
 */

use std::io;
use std::io::Write;

fn main(){ 

    print!("Enter the length of the rectangle: ");
    let length = string_to_float();
    print!("Enter the width of the rectangle: ");
    let width = string_to_float();

    let perimeter = rectangle_perimeter(length, width);
    println!("Length:       {:.3} Units", length);
    println!("Width:        {:.3} Units", width);
    println!("Perimeter:    {:.3} Units", perimeter);

} 

fn string_to_float() -> f32{

    let mut string_float_buffer = String::new();

    io::stdout()
        .flush()
        .unwrap();

    io::stdin()
        .read_line(&mut string_float_buffer)
        .expect("Error: cannot read line.");

    let string_float_buffer: f32 = string_float_buffer
        .trim()
        .parse()
        .expect("Error: cannot convert string to float.");

return string_float_buffer;
}

fn rectangle_perimeter(length_value: f32, width_value: f32) -> f32{

    let perimeter_value = 2.0 * (length_value + width_value);

return perimeter_value;
}
