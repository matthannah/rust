// Since closures may be used as arguments, you might wonder if the same can be
// said about functions. They CAN...

// If you declare a function that takes a closure as a parameter, then any function
// that satisfies the trait bound of that closure can be passed as a parameter.

// Define a function which takes a generic 'F' argument
// bounded by 'Fn', and calls it:

// can be written like this:
// fn function<F>(f: F) where F: Fn() {
// or sugar:
fn call_me<F: Fn()>(f: F) {
    f();
}

// A function that accepts a function bounded by FnMut()
/* fn call_me_2<F: FnMut()>(mut f: F) {
    f();
}*/

fn function() {
    println!("I'm a function!");
}

fn main() {
    let /*mut*/ x = 1_u8;

    // Define a closure satisfying the 'Fn' bound
    let closure = || {
        // x += 1;
        // TODO^ Uncomment out that line and the closure no longer satisfies 'Fn'
        // If you comment it out and pass it into call_me_2 it will work because
        // it is bounded by FnMut
        println!("The value of x is: {}", x);
    };

    call_me(function);
    call_me(closure);
    // call_me_2(closure);
}
