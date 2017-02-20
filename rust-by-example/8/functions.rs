// Functions are declared using the 'fn' keyword. Its arguments are type annotated,
// just like variables, and, if the function returns a value, the return type must be
// specified after an arrow '->'

// The final expression in the function will be used as return value. Alternatively,
// the return statement can be used to return a value earlier from within the function,
// even from inside loops or if's.

// Rewrite FizzBuzz using functions:

// Unlike C/C++ there's no restriction to the order of function definitions
fn main() {
    // we can use this function and define it later
    fizzbuzz_to(100);
}

// Functions that "don't" return a value, actually return the unit type `()`
fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

// function returns a bool
fn is_divisible_by(x: u32, y: u32) -> bool {
    // make sure to not divide by zero:
    if y == 0 { return false; } // return early

    x % y == 0
}

// When a function returns (), the return type can be omitted from the signature
// no '-> type'
fn fizzbuzz_to(n: u32) {
    for n in 1 .. n + 1 { // we can use n again because we are rebinding it each loop
        fizzbuzz(n);
    }
}
