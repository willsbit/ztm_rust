// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

enum Position {
    Maintenance,
    Marketing,
    Manager,
    LineSupervisor,
    Kitchen,
    AssemblyTech
}

struct Employee {
    position: Position,
    employed: bool
}


fn can_enter(employee: &Employee) -> Result<bool, String> {
    if employee.employed == true {
        match employee.position {
            Position::Maintenance => Ok(true),
            Position::Marketing => Ok(true),
            Position::Manager => Ok(true),
            _ => Err("INSUFFICIENT CREDENTIALS - NOT AUTHORIZED".to_owned())
        }
    } else {
        return Err("TERMINATED EMPLOYEE - NOT AUTHORIZED".to_owned());
    }
}

fn print_status(employee: &Employee) -> Result<(), String> {
    let access_status = can_enter(employee)?;
    println!("Acess authorized");
    Ok(())
}

fn main() {
    let test_employee = Employee {
        position: Position::Kitchen,
        employed: true
    };
    match print_status(&test_employee) {
        Err(e) => println!("access denied: {:?}", e),
        _ => (),
    }
}
