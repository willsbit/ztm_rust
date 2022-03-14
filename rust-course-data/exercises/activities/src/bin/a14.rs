// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct KidCharacteristics {
    age: i32,
    name: String,
    color: String
}

impl KidCharacteristics {
    fn print(&self){
        println!("Name: {:?}", self.name);
        println!("Color: {:?}", self.color);
    }
}

fn main() {
    let people = vec![
        KidCharacteristics{ age: 9, name: String::from("Alice"), color: String::from("Yellow") },
        KidCharacteristics{ age: 6, name: String::from("Bob"), color: String::from("Red") },
        KidCharacteristics{ age: 4, name: String::from("Charles"), color: String::from("Blue") },
    ];

    for person in people{
        println!("Age: {:?}", person.age);
        person.print();
        println!("\n");
    }
}
