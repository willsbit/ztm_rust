// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Colors {
    Yellow,
    Blue,
    Red,
    Green
}

fn color_name(color:Colors) {
    match color {
        colors::Yellow => {println!("yellow")}
        colors::Blue => {println!("blue")}
        colors::Red => {println!("red")}
        colors::Green => {println!("green")}
    }
}

fn main() {
    color_name(Colors::Red)
}
