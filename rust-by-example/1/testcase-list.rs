// Implementing fmt::Display for a structure where the elements must
// be handled sequentially is tricky. The problem is that each write!
// generates a fmt::Result. Proper handling of this requires dealing with all
// the results. Rust provides the try! macro for exactly this purpose

// Try write! to see if it errors. If it errors, return the error, continue otherwise
// try!(write!(f, "{}", value));

// with try! available implementing fmt::Display for a Vec is straightforward

use std::fmt; // import the fmt module

struct List(Vec<i32>); // structure named list containing a Vec

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Dereference self and create a reference to Vec
        // via destructuring
        let List(ref vec) = *self;

        try!(write!(f, "["));

        // Iterate over 'vec' in 'v' while enumerating the iteration count in 'count'
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma before calling the
            // 'write!'. Use 'try!' to return on errors.
            if count !=0 { try!(write!(f, ", ")); }
            try!(write!(f, "{}", v));
        }

        // Close the opened bracket and return a fmt::Result value
        write!(f, "]")
    }
}

// ACTIVITY
// Try changing the program so that the index of each element in the vector is also printed.
// The new output should look like this:
// [0: 1, 1: 2, 2: 3]

struct List2(Vec<i8>);

// Implement fmt::Display
impl fmt::Display for List2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let List2(ref vec) = *self;

        // Open bracket
        try!(write!(f, "["));

        for (count, vec) in vec.iter().enumerate() {
            if count != 0 { try!(write!(f, ", ")); }
            try!(write!(f, "{}: {}", count, vec));
        }

        write!(f, "]")
        // Close bracket, return the fmt::Result
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);

    let v2 = List2(vec![1, 2, 3]);
    println!("{}", v2);
}
