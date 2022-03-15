// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info
enum Tickets {
    Backstage(String, f64),
    Vip(String, f64),
    Standard(f64)
}


fn main() {
    let ticket_list = vec![Tickets::Backstage("Will".to_owned(), 30.00),
                                      Tickets::Vip("Carol".to_owned(), 30.00),
                                      Tickets::Standard(15.00)];

    for ticket in ticket_list {
        match ticket {
            Tickets::Backstage(owner, price) =>
                println!("The owner of this Backstage ticket is {:?} and it costs {:?}", owner, price),
            Tickets::Vip(owner, price) =>
                println!("The owner of this VIP ticket is {:?} and it costs {:?}", owner, price),
            Tickets::Standard(price) => println!("This Standard ticket costs {:?}", price),
        }
    }
}
