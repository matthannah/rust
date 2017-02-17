fn main() {
    // In general, the '{}' will be automatically replaced with any
    // arguments. They will be stringified.
    println! ("I have {} flowers", 50);

    // Without a suffix, the above becomes an i32. You can change what type 50 is,
    // with a suffix:

    // suffix's include:
    // i8, i16, i32, i64 - signed numbers upto bits
    // u8, u16, u32, u64 - unsigned numbers upto bits
    // isize, usize      - variable size which depends on underlying machine architecture
    // f32, f64          - floating point corresponding to IEEE-754 single/double precision

    println! ("I have {} flowers", 50u16); // unsigned int of 16 bits

    // obviously -50u16 will throw an error because it's a signed number we are trying
    // to declare
    // additionally 1000i8 will throw a warning and print the incorrect numbers
    // because it's too big (out of range)!

    // There are various patterns that can be used:
    println! ("{0} {1} {0}", 1, 2); // prints 1 2 1

    // named arguments can be used too!
    println! ("My dogs name is {dog}. My name is {me}", dog="Ness", me="Matt");

    // special formatting can be specified after a ':'
    println! ("{} of {:b} people know binary, the other half don't", 1, 2);
    // :b converted 2 to binary

    // You can right align text with a specified width
    println! ("{number:>width$}", number=1, width=10);

    // or pad with zeroes
    println! ("{number:>0width$}", number=1, width=4);

    // --- FIX ME ---
    // It will make sure that the number of arguments are being used
    // println! ("{name} {lastname}", name=matt); // add in lastname

    // create a structure which contains an 'i32'. Name it 'Structure'
    #[allow(dead_code)]
    struct Structure(i32);

    // --- FIX ME ---
    // The structure can't be printed, needs more complicated handling
    // println! ("This struct {} won't print...", Structure(3));
    // In order to print fmt::Display will need to be implemented.

    // FIXES

    // implementing the format trait
    impl std::fmt::Display for Structure {

        // We need to implement the method of the signature:
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            // type is passed as self

            // the return value of this function is fmt::Result
            //       - which is a type alias of Result<(), std::fmt::Error>

            // the 'f' value implements the 'Write' trait, which is what the
            // write! macro is expecting. Note that this formatting ignores the
            // various flags provided to the format strings.
            write!(f, "{}", self.0)
        }
    }

    println! ("This struct {} will print...", Structure(3));

    // Display 3 decimals of pi
    println! ("Pi is roughly {pi:.3}", pi=3.141592);

}
