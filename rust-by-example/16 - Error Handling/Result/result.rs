// Result is a richer version of the Option type that describes possible error instead of possible
// absence.

// That is, Result<T, E> could have one of two outcomes:

// Ok<T>: An element T was found
// Err<E>: An error was found with element E
// By convention, the expected outcome is Ok while the unexpected outcome is Err.

// Like Option, Result has many methods associated with it. unwrap(), for example, either yields
// the element T or panics. For case handling, there are many combinators between Result and
// Option that overlap.

// In working with Rust, you will likely encounter methods that return the Result type, such as
// the parse() method. It might not always be possible to parse a string into the other type, so
// parse() returns a Result indicating possible failure.

// Let's see what happens when we successfully and unsuccessfully parse() a string:

fn double_number(number_str: &str) -> i32 {
    // Let's try using `unwrap()` to get the number out. Will it bite us?
    2 * number_str.parse::<i32>().unwrap()
}

fn main() {
    let twenty = double_number("10");
    println!("double is {}", twenty);

    let tt = double_number("t");
    println!("double is {}", tt);
}

// In the unsuccessful case, parse() leaves us with an error for unwrap() to panic on.
// Additionally, the panic exits our program and provides an unpleasant error message.

// To improve the quality of our error message, we should be more specific about the return type
// and consider explicitly handling the error.
