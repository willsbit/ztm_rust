// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase
#[derive(Debug)]
struct Customer {
    name: String,
    age: i32
}

fn can_purchase(customer: &Customer) -> Result<bool, String> {
    if customer.age < 21 {
        Err("customer must be at least 21 years old".to_owned())
    } else {
        Ok(true)
    }
}

fn main() {
    let will = Customer {
        name: "Will".to_owned(),
        age: 25
    };

    let purchased = can_purchase(&will);
    println!("{:?}", purchased);
    // pick_choice(&will);

}