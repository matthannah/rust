// It's possible to declare variable bindings first, and initialize them later.
// However, this form is seldom used, as it may lead to the use of unitialized variables.

fn main() {
    // Declare a variable binding
    let a_binding;

    {
        let x = 2;

        // Initialize the binding
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    // Error! Use of uninitialized binding
    // println!("another binding: {}", another_binding);
    // FIXME ^ Comment out this line

    another_binding = 1;

    println!("another binding: {}", another_binding);
}

// The compiler forbids the use of unitialized bindings as it causes undefined behaviour
