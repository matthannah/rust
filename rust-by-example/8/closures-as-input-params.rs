// While rust chooses how to capture variables on the fly mostly without type annotation,
// this ambiguity is not allowed when writing functions. When taking a closure as
// an input parameter, the closure's complete type must be annotated using one of a
// few traits:

// Fn: the closure captures by reference - &T
// FnMut: the closure captures by mutable reference - &mut T
// FnOnce: the closure captures by value - T

// On a variable-by-variable basis, the compiler will capture variables in the least
// restrictive manner possible.

// For instance, consider a parameter annotated as FnOnce. This specifies that the
// closure may capture by T, &T or &mut T, but the compiler will ultimately choose
// based on how the captured variables are used in the closure.

// This is because if a move is possible, then any type of borrow should also be
// possible. Note that the reverse is NOT true. If the parameter is annotated as Fn,
// then capturing variables by &mut T ot T are not allowed.

// In the following example, try swapping the usage of Fn, FnMut and FnOnce to see what
// happens:

// A function which takes a closure as an argument and calls it.
// The closure takes no input and returns nothing
fn apply<F>(f: F) where F: FnOnce() {
    // fn apply<F>(mut f: F) where F: FnMut() {
    // ^ TODO: Try changing this to `Fn` or `FnMut`.
    f();
}

// A function which takes a closure and returns an `i32`.
// The closure takes an `i32` and returns an `i32`.
fn apply_to_3<F>(f: F) -> i32 where F: Fn(i32) -> i32 {
    f(3)
}

fn main() {
    use std::mem;

    let greeting = "hello";
    // A non-copy type.
    // `to_owned` creates owned data from borrowed one
    let mut farewell = "goodbye".to_owned();

    // Capture 2 variables: `greeting` by reference and
    // `farewell` by value.
    let diary = || {
        // `greeting` is by reference: requires `Fn`.
        println!("I said {}.", greeting);

        // Mutation forces `farewell` to be captured by
        // mutable reference. Now requires `FnMut`.
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // Manually calling drop forces `farewell` to
        // be captured by value. Now requires `FnOnce`.
        mem::drop(farewell);
    };

    // Call the function which applies the closure.
    apply(diary);

    // `double` satisfies `apply_to_3`'s trait bound
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}
