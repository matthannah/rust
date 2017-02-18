// fmt::Debug hardly looks compact and clean... So it's often desirable to
// manually implement the fmt::Display which uses the {} print marker.

// We can import the 'fmt' module to make it available
use std::fmt;

// Defining a structure which 'fmt::Display' will be implemented for!
// This is just an empty tuple struct containing a 'i32' bound to the name
// Structure.
struct Structure(i32);

// So in order to use the {} marker with our new type, we must manually implement
// the 'fmt::Display'

impl fmt::Display for Structure {
    // This trait requires 'fmt' with this exact signature:
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.0)
    }
}

// fmt::Display may be cleaner than fmt::Debug but this presents a problem for the std
// library. How should ambiguous types be displayed? For example if the std library
// implemented a single style for all 'Vec<T>', what style should it be?

// Either of these 2?
//  Vec<path>:   /:/etc:/home/username:/bin (split on :)
//  Vec<number>: 1,2,3 (split on ,)
// No.

// There is no ideal style for all types and the std library doesn't presume to
// dictate one. fmt::Display is not implemented for 'Vec<T>' or for any other generic
// containers. fmt::Debug must then be used for these cases.

// This is not a problem though because for any new container type which is not generic,
// fmt::Display can be implemented.

#[derive(Debug)] // Note that we are automatically generating :? output for this struct
struct MinMax(i64, i64);

// Implement 'Display' for 'MinMax'
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // use self.number to refer to each positional data point
        write!(f, "min: {}, max: {}", self.0, self.1)
    }
}

// Define a structure where the fields are nameable for comparison.
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// Similarly, implement 'Display' for 'Point2D'
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// ACTIVITY
// Create Complex struct with output printed like the following:
// Display: 3.3 + 7.2i
// Debug:   Complex { real: 3.3, imag: 7.2 }
#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range   = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // Error. Both `Debug` and `Display` were implemented but `{:b}`
    // requires `fmt::Binary` to be implemented. This will not work.
    // println!("What does Point2D look like in binary: {:b}?", point);

    // ACTIVITY - Complex
    let complex = Complex { real: 3.3, imag: 7.2 };

    println!("{}", complex);
    println!("{:?}", complex);
}
