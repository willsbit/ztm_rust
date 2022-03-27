// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None
    } else{
        Some(a / b)
    }
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn check_clamp() {
        let n = 5;
        let lower = 5;
        let upper = 10;
        assert_eq!(clamp(n, lower, upper), n, "CLAMP DOES NOT WORK");
    }

    #[test]
    fn check_clamp2() {
        let n = 3;
        let lower = 5;
        let upper = 10;
        assert_eq!(clamp(n, lower, upper), lower, "CLAMP DOES NOT WORK");
    }

    #[test]
    fn check_div() {
        let a = 6;
        let b = 2;
        let expected = Some(3);
        assert_eq!(div(a, b), expected, "div error");
    }

    #[test]
    fn check_div2() {
        let a = 1;
        let b = 0;
        let expected = None;
        assert_eq!(div(a, b), expected, "div error");
    }

    #[test]
    fn check_concat() {
        let first = "abc";
        let second = "def";
        let expected = "abcdef";
        assert_eq!(concat(first, second), expected, "concat error");
    }

    #[test]
    fn check_concat2() {
        let first = "4d5s4dad545{:?}";
        let second = "{:?}//panic!";
        let expected = "4d5s4dad545{:?}{:?}//panic!";
        assert_eq!(concat(first, second), expected, "concat error");
    }

}
