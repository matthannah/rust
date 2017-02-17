fn main() {
    // In general, the '{}' will be automatically replaced with any
    // arguments. They will be stringified.
    println! ("I have {} flowers", 50);

    // Without a suffix, the above becomes an i32. You can change what type 50 is,
    // with a suffix:

    // some suffix's include:
    // i8, i16, i32, i64 - signed numbers upto bits
    // u8, u16, u32, u64 - unsigned numbers upto bits
    // isize, usize      - variable size which depends on underlying machine architecture
    // f32, f64          - floating point corresponding to IEEE-754 single/double precision

    println! ("I have {} flowers", 50u16); // unsigned int of 16 bits

    // obviously -50u16 will throw an error because it's a signed number we are trying
    // to declare
    // additionally 1000i8 will throw a warning and print the incorrect numbers
    // because it's too big (out of range)!

    
}
