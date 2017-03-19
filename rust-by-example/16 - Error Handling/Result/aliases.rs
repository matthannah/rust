// How about when we want to reuse a specific Result type many times? Recall that Rust allows us
// to create aliases. Conveniently, we can define one for the specific Result in question.

// At a module level, creating aliases can be particularly helpful. Errors found in a specific
// module often have the same Err type, so a single alias can succinctly define all associated
// Results. This is so useful that the std library even supplies one: io::Result!

// Here's a quick example to show off the syntax:

use std::num::ParseIntError;

// Define a generic alias for a `Result` with the error type `ParseIntError`.
type AliasedResult<T> = Result<T, ParseIntError>;

// Use the above alias to refer to our specific `Result` type.
fn double_number(number_str: &str) -> AliasedResult<i32> {
    number_str.parse::<i32>().map(|n| 2 * n)
}

// Here, the alias again allows us to save some space.
fn print(result: AliasedResult<i32>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    print(double_number("10"));
    print(double_number("t"));
}
