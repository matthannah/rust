//! This is a file showing how comments are used

fn main() {
    // This is a line comment
    /* This is a block comment */

    let x = 5 + /* 90 + */ 5;
    println!("x is {}", x); // should be 10
}

/// A human being is represented here
pub struct Person {

}
