// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.

use std::collections::HashMap;
use std::io;
#[derive(Debug, Clone)]
pub struct Bill {
    name: String,
    amount: f64,
}

pub struct Bills {
    inner: HashMap<String, Bill>
}
impl Bills {
    fn new() -> Self {
        Self {
            inner: HashMap::new()
        }
    }
    fn add(&mut self, bill: Bill) {
        self.inner.insert(bill.name.to_string(), bill);
    }
    fn get_all(&self) -> Vec<&Bill> {
        self.inner.values().collect()
    }
    fn remove(&mut self, name: &str) -> bool {
        self.inner.remove(name).is_some()
    }
}

fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while std::io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter a valid input");
    }
    let input = buffer.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}

fn get_bill_amount() -> Option<f64> {
    println!("Amount: ");
    loop {
        let input = match get_input() {
            Some(input) => input,
            None => return None,
        };
        if &input == "" {
            return None
        };

        let parsed_input: Result<f64, _> = input.parse();
        match parsed_input{
            Ok(amount) => return Some(amount),
            Err(_) => println!("Please enter a number")
        }
    }
}


enum MainMenu {
    AddBill,
    ViewBill,
    RemoveBill
}
impl MainMenu {
    fn from_str(input: &str) -> Option<MainMenu> {
        match input {
            "1" => Some(MainMenu::AddBill),
            "2" => Some(MainMenu::ViewBill),
            "3" => Some(MainMenu::RemoveBill),
            _ => None,
        }
    }
    fn show_options() {
        println!("========= Bill Manager =======");
        println!("Choose your desired action:");
        println!("1. Add a bill");
        println!("2. View bills");
        println!("3. Remove a bill");
        println!("4. Edit a bill");
        println!("5. Return to main menu");
        println!();
        println!("Enter selection:");
    }
}


mod menu {
    use crate::{Bill, Bills, get_bill_amount, get_input};

    pub fn add_bill(bills: &mut Bills) {
        println!("Bill name: ");
        let name = match get_input() {
            Some(input) => input,
            None => return
        };

        let amount = match get_bill_amount() {
            Some(amount) => amount,
            None => return
        };

        let bill = Bill{name, amount};
        bills.add(bill);
        println!("Bill added!");
    }

    pub fn view_bills(bills: &Bills) {
        for bill in bills.get_all() {
            println!("{:?}", bill)
        }
    }

    pub fn remove_bills(bills: &mut Bills) {
        for bill in bills.get_all() {
            println!("{:?}", bill)
        }
        println!("Enter bill name to remove: ");

        let name = match get_input() {
            Some(name) => name,
            None => return
        };

        if bills.remove(&name) {
            println!("Bill removed.")
        } else {
            println!("Bill not found.")
        }
    }
}


fn main() {

    let mut bills = Bills::new();

    loop {
        MainMenu::show_options();
        let input = get_input().expect("No data entered");
        match MainMenu::from_str(input.as_str()) {
            None => return,
            Some(MainMenu::AddBill) => menu::add_bill(&mut bills),
            Some(MainMenu::ViewBill) => menu::view_bills(&bills),
            Some(MainMenu::RemoveBill) => menu::remove_bills(&mut bills)
        }
    }
}
