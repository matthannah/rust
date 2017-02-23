fn main() {
    let mut x = 10_u8; // create a mutable u8
    increment(&mut x); // pass a mutable reference
    println!("{}", x) // print x
}

fn increment(x: &mut u8) { // accept a mutable reference to a u8
    *x += 1; // dereference and increment by 1
}
