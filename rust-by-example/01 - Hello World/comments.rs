//! Generate library docs for the enclosing item

fn main() {
    // This is a line comment
    /* This is a block comment */

    let x = 5 + /* 90 + */ 5;
    println!("x is {}", x); // should be 10
}

/// Generate library docs for the following item
pub struct Person {

}
