// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter

trait Perimeter {
    fn calculate_perimeter(&self) -> i32;
}

struct Square {
    side: i32
}

impl Perimeter for Square {
    fn calculate_perimeter(&self) -> i32 {
        let p = self.side * 4;
        println!("The perimeter of the square is {:?}", p);
        p
    }
}

struct Triangle {
    a: i32,
    b: i32,
    c: i32
}

impl Perimeter for Triangle {
    fn calculate_perimeter(&self) -> i32 {
        let p = self.a + self.b + self.c;
        println!("The perimeter of the triangle is {:?}", p);
        p
    }
}

fn perimeter(geometry: impl Perimeter) {
    Perimeter::calculate_perimeter(&geometry);
}

fn main() {
    let sqr = Square {side: 30};
    let triangle = Triangle {a: 3, b:4, c:5};

    perimeter(sqr);
    perimeter(triangle);
}


