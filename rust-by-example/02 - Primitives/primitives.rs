// Rust provides a wide variety of primitives. A sample includes:
// signed integers
// unsigned integers
// floating point
// char - unicode scalar values like 'a' 'b' etc... 4 bytes each.
// bool - true / false
// unit type () - whose only value is also ()
// arrays [1, 2, 3]
// tuples like (1, true)

// Variables can always be type annotated. Numbers may additionally be annotated
// via a suffix or by default. Integers default to 132 and floats to f64

fn main() {
    // Variables can be type annotated.
    let logical: bool = true;

    let a_float: f64 = 1.0;  // Regular annotation
    let an_integer   = 5i32; // Suffix annotation

    // Or a default will be used.
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`

    // Error! default_integer cannot be mutated!
    // default_integer = 4

    let mut mutable = 12; // Mutable `i32`.
    mutable = 10; // the 'mut' keyword allows us to change what the binding
                  // 'mutable' points to

    // Error! The type of a variable can't be changed
    // mutable = true;
}
