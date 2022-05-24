// Topic: Smart Pointers & RefCell
//
// Summary:
//   A vehicle rental company wants to access the rentals available
//   at storefront locations. Create a program that provides access
//   to storefront rentals from the corporate headquarters.
//
// Requirements:
// * Corporate must be able to access the rentals at a storefront
// * Storefronts must be able to rent out vehicles
// * Rentals have the following attributes:
//   - Type of vehicle
//   - Vehicle Identification Number (VIN)
//   - Vehicle status:
//     * Available, Unavailable, Maintenance, Rented
//
// Notes:
// * Use Rc and RefCell to create shared mutable data structures
// * Create at least two rentals and ensure that Corporate and StoreFront
//   can both access the rental information
// * Test your program by changing the vehicle status from both a storefront
//   and from corporate
use std::rc::Rc;
use std::cell::RefCell;
#[derive(Debug)]
enum RentalStatus {
    Available,
    Unavailable,
    Maintenance,
    Rented,
}
#[derive(Debug)]
struct Rental{
    vehicle_type: String,
    vin: String,
    status: RentalStatus,
}

type RentalPtr = Rc<RefCell<Rental>>;

#[derive(Debug)]
struct Corporate(RentalPtr);
#[derive(Debug)]
struct StoreFront(RentalPtr);

fn main() {
    let rental1 = Rc::new(RefCell::new(Rental {
        vehicle_type: "Uno".to_string(),
        vin: "12345".to_string(),
        status: RentalStatus::Available,
    }));

    let corporate = Corporate(Rc::clone(&rental1));
    let store_front = StoreFront(Rc::clone(&rental1));

    let rental2 = Rc::new(RefCell::new(Rental {
        vehicle_type: "MOBI".to_string(),
        vin: "321654".to_string(),
        status: RentalStatus::Available,
    }));

    let corporate2 = Corporate(Rc::clone(&rental2));
    let store_front2 = StoreFront(Rc::clone(&rental2));

    dbg!(corporate.0.borrow());
    dbg!(store_front.0.borrow());

    dbg!(corporate2.0.borrow());
    dbg!(store_front2.0.borrow());

    // Change the status of the vehicle
    rental2.replace(Rental {
        vehicle_type: "Strada".to_string(),
        vin: "99999".to_string(),
        status: RentalStatus::Rented,
    });

    dbg!(corporate2.0.borrow());
    dbg!(store_front2.0.borrow());
}
