// Topic: Generics & Structures
//
// Requirements:
// * Create a Vehicle structure that is generic over traits Body and Color
// * Create structures for vehicle bodies and vehicle colors and implement the
//   Body and Color traits for these structures
// * Implement a 'new' function for Vehicle that allows it to have any body
//   and any color
// * Create at least two different vehicles in the main function and print their
//   info
//
// Notes:
// * Examples of car bodies can be Truck, Car, Scooter
// * Examples of colors could be red, white, black
// * It is not necessary to have data fields or function implementations
//   for the vehicle bodies/colors

trait Body {}
trait Color {}
#[derive(Debug)]
struct Vehicle<B:Body, C:Color> {
    body: B,
    color: C
}

impl<B:Body, C:Color> Vehicle<B,C> {
    pub fn new(body: B, color: C) -> Self {
        Self { body, color }
    }
}
#[derive(Debug)]
struct Red;
impl Color for Red {}
#[derive(Debug)]
struct White;
impl Color for White {}

#[derive(Debug)]
struct Scooter;
#[derive(Debug)]
struct Car;
impl Body for Scooter {}
impl Body for Car {}
fn main() {

    let mobi = Vehicle::new(Car, White);
    let cg = Vehicle::new(Scooter, Red);

    println!("O mobi tem body {:?} e color {:?}", mobi.body, mobi.color);
    println!("{:?}", cg);
}
