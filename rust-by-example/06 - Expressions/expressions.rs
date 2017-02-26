// A Rust program is (mostly) made up of a series of statements:

fn main() {
    // statement
    // statement
    // statement

    // There are a few kinds of statements in Rust. The most common two are declaring
    // a variable binding. and using the ';' with an expression:

    // variable binding
    let x = 2;

    // expression
    x;
    x + 1;
    15;

    // Blocks are expressions too, so they can be used as r-values in assignments.
    // The last expression in the block will be assigned to the l-value. However,
    // if the last expression of the block ends with a semicolon, the return value
    // will be ().

    let y = 5_u32;

    let z = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expresssion will be assigned to 'z'
        x_cube + x_squared + x
    };

    let i = {
        // The semicolon supresses the expression and '()' is assigned to 'i'
        2 * x;
    };

    println!("x is: {:?}", x);
    println!("y is: {:?}", y);
    println!("z is: {:?}", z);
    println!("i is: {:?}", i);
}
