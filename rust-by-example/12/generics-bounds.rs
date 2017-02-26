// When working with generics, the type parameters often must use traits as bounds
// to stipulate what functionality a typle implements. For example, the following
// example uses the trait Display to print and so it requires <T> to be bound by
// Display: that is, T ..MUST.. implement Display.

// Define a function `printer` that takes a generic type `T` which
// must implement trait `Display`.
fn printer<T: std::fmt::Display>(t: T) {
    println!("{}", t);
}

// Bounding restricts the generic to types that conform to the bounds:

struct S<T: std::fmt::Display>(T);

// Error! `Vec<T>` does not implement `Display`. This
// specialization will fail.
fn error_example() {
    // let s = S(vec![1]);
}

// Another effect of bounding is that generic instances are allowed to access the
// methods of traits specified in the bounds:

// A trait which implements the print marker: `{:?}`.
use std::fmt::Debug;

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 { self.length * self.height }
}

#[derive(Debug)]
struct Rectangle { length: f64, height: f64 }
#[allow(dead_code)]
struct Triangle  { length: f64, height: f64 }

// The generic `T` must implement `Debug`. Regardless
// of the type, this will work properly.
fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

// `T` must implement `HasArea`. Any function which meets
// the bound can access `HasArea`'s function `area`.
fn area<T: HasArea>(t: &T) -> f64 { t.area() }

fn main() {
    let rectangle = Rectangle { length: 3.0, height: 4.0 };
    let _triangle = Triangle  { length: 3.0, height: 4.0 };

    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));

    //print_debug(&_triangle);
    //println!("Area: {}", area(&_triangle));
    // ^ TODO: Try uncommenting these.
    // | Error: Does not implement either `Debug` or `HasArea`.
}

// As an additional note, where clauses can also be used to apply bounds
// in some cases to be more expressive.
