// Variable bindings are immutable by default, but this can be overriden using
// the mut modifier.

fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // Ok
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // Error!
    // _immutable_binding = 1;
    // FIXME^ Comment out this line
}

// The compiler will throw detailed diagnostics about mutability errors.
