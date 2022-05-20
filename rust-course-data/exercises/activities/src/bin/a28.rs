// Topic: New type pattern
//
// Requirements:
// * Display the selected color of shoes, a shirt, and pants
// * Create and display at least one of each type of clothes and color
//
// Notes:
// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function
// * Create a function for each type of clothes (shoes, shirt, pants)
//   that accepts the new type specific to that type of clothing
struct Shirt(Color);
struct Shoes(Color);
struct Pants(Color);

impl Shirt {
    fn new(color: Color) -> Self {
        Self(color)
    }
}
impl Shoes {
    fn new(color: Color) -> Self {
        Self(color)
    }
}
impl Pants {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

#[derive(Debug)]
enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}


fn show_color(pants: Pants) {
    println!("These pants are color {:?}", pants.0);
}

fn wax_shoes(shoes: Shoes) {
    println!("Freshly waxed shoes with color {:?}", shoes.0);
}


fn rate_shirt(shirt: Shirt) {
    println!("The color {:?} looks great on that shirt!", shirt.0);
}

fn main() {
    let black_pants = Pants::new(Color::Black);
    let black_shoes = Shoes::new(Color::Black);
    let black_shirt = Shirt::new(Color::Black);

    show_color(black_pants);
    wax_shoes(black_shoes);
    rate_shirt(black_shirt);
}
