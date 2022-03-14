// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics
struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}
#[derive(Debug)]
enum Color {
    Blue,
    Yellow,
    Red
}

struct Box {
    dimensions: Dimensions,
    weight: f64,
    color: Color
}

impl Box {
    fn create_box(weight:f64, color:Color, dimensions:Dimensions) -> Self {
        Self {
            dimensions,
            weight,
            color
        }
    }
    fn print_characteristics(&self) {
        println!("The box has dimensions (w-h-d): {:?} x {:?} x {:?}m",
                 self.dimensions.width, self.dimensions.height, self.dimensions.depth);
        println!("The box weighs {:?} kg", self.weight);
        println!("The box is {:?}", self.color);
    }

}
fn main() {
    let my_box = Box::create_box(2.5, Color::Blue, Dimensions {width:2.5, height:3.0, depth:1.0});
    my_box.print_characteristics()
}
