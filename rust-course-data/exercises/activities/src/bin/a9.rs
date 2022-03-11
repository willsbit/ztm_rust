// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn cartesian_coords_generator(x: i32, y: i32) -> (i32, i32) {
    let (x, y) = (x, y);
    if y > 5 {
        println!("The y-value is greater than 5");
        (x, y)
    } else if y == 5 {
        println!("The y-value is 5");
        (x, y)
    } else {
        println!("The y-value is greater than 5");
        (x, y)
    }
}

fn main() {
    cartesian_coords_generator(5 ,6);
}
