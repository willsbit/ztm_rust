// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    name: String,
    locker_assignment: Option<i32>
}

fn main() {
    let will = Student {
        name: "Willian".to_owned(),
        locker_assignment: Some(23)
    };

    match will.name {
        name => println!("The student is {:?}", name)
    };
    match will.locker_assignment {
        Some(ans) => println!("The locker assignment is {:?}", ans),
        None => println!("The student has no locker")
    };
}
