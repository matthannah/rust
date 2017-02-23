// Closures as input parameters are possible, so returning closures as output params
// should also be possible. However, returning closure types are problematic because
// Rust currently only supports returning conrete (non-generic) types. Anonymous
// closure types, are by definition, unknown and so returning a closure is only possible
// by making it concrete. This can be done via boxing.

// The valid traits for returns are slightly different from before:
// Fn     : normal
// FnMut  : normal
// FnOnce : There are some unusual things at play here, so the FnBox type is currently
//          needed, and is unstable. This is expected to change in the future.

// Beyond this, the move keyword must be used, which signals that all captures occur
// by value. This is required because any captures by reference would be dropped as
// soon as the function exited, leaving invalid references in the closure.

fn create_fn() -> Box<Fn()> {
    let text = "Fn".to_owned();

    Box::new(move || println!("This is a: {}", text)) // returns a function bounded to
                                                      // trait Fn
}

fn create_fnmut() -> Box<FnMut(u8)> {
    let text = "FnMut".to_owned();

    Box::new(move |mut x| {
        println!("x was: {}", x);
        x += 1;
        println!("This is a: {}", text);
        println!("x is now: {}", x)
    }) // returns a function bounded to
                                                      // trait FnMut
}

fn main() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();

    fn_plain();
    let x = 1;
    fn_mut(x);
    println!("{}", x)
}
