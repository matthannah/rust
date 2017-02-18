// All types that want to use the std::fmt traits must require an implementation
// in order to be printable
// Automatic implementations are only available for types provided in the
// std library

// The fmt::Debug trait makes this very straightforward. All types can derive the
// fmt::Debug implementation. This is not true for fmt::Display however, which must
// be manually implemented

// This struct cannot be printed either with 'fmt::Display' or 'fmt::Debug'
// struct Unprintable(i32);

// The 'derive' attribute automatically creates the implementation
// required to make this 'struct' printable with 'fmt::Debug'
// #[derive(Debug)]
// struct DebugPrintable(i32);

// All the standard library types are printable with {:?} too. (Debug)

// Derive the `fmt::Debug` implementation for `Structure`. `Structure`
// is a structure which contains a single `i32`.
#[derive(Debug)]
struct Structure(i32);

// Put a `Structure` inside of the structure `Deep`. Make it printable
// also.
#[derive(Debug)]
struct Deep(Structure);

fn main() {
    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));

    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));

    // So fmt::Debug definitely makes this printable but sacrifices some elegance.
    // Manually implementing fmt::Display will fix that.
}
