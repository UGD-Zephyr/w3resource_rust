/* Programmer:          Per Stoor
 * Date:                2023-04-25
 * Last changed:        2023-04-25
 * Type of program:     
 */

fn main(){ 

    let array1: [i32; 5] = [1, 2, 3, 4, 5];
    let mut total_loops: usize = 0;

    /*
     * Other valid options for printing an array on the screen.
     *
    println!("{:?}", array1);
    println!("{:#?}", array1);
    */

        for index in 0..array1.len(){
            print!("{} ", array1[index]);
            total_loops = index + 1;
        }
        println!("Total loops: {}", total_loops);
        println!();

} 
