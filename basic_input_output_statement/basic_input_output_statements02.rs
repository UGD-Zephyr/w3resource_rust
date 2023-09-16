/* Programmer:          Per Stoor
 * Date:                2023-09-16
 * Last changed:        2023-09-16
 * Type of program:     Program that calculates the volume of a sphere.
 */

use std::io;
use std::io::Write;
use std::f64::consts::PI;

fn main(){ 

    print!("Enter the radius of a sphere: ");
    let radius = string_to_float();
    let volume = sphere_volume(radius);

    println!("Radius: {:.2} units", radius);
    println!("Volume: {:.6} units", volume);

} 

fn string_to_float() -> f64{

    let mut string_float_buffer = String::new();
    io::stdout()
        .flush()
        .unwrap();
    io::stdin()
        .read_line(&mut string_float_buffer)
        .expect("Error: io::stdin()");

    let string_float_buffer: f64 = string_float_buffer
        .trim()
        .parse()
        .expect("Error: Can't turn string to a float.");

return string_float_buffer;
}

fn sphere_volume(radius_value: f64) -> f64{

    let volume_value = (4.0/3.0) * PI * radius_value.powi(3);

return volume_value;
}
