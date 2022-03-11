// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor
#[derive(Debug)]
enum DrinkFlavors {
    Cola,
    Lemon,
    Strawberry,
    Pineapple
}

struct Drink{
    flavor:DrinkFlavors,
    fl_oz: f64
}

fn show_drink(drink: Drink) {
    println!("Fluid ounces: {:?}", drink.fl_oz);
    match drink.flavor {
        DrinkFlavors::Cola => {println!("It's a cola flavored drink!")}
        DrinkFlavors::Lemon => {println!("It's a lemon flavored drink!")}
        DrinkFlavors::Strawberry => {println!("It's a strawberry flavored drink!")}
        DrinkFlavors::Pineapple => {println!("It's a pineapple flavored drink!")}
    }
}

fn main() {
    let my_drink = Drink {
        flavor: DrinkFlavors::Lemon,
        fl_oz: 25.50
    };
    show_drink(my_drink)
}
