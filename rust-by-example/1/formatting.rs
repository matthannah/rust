// We've seen that formatting is specified via a format string:

// format!("{}", foo) -> "3735928559"
// format!("0x{:X}", foo) -> "0xDEADBEEF"
// format!("0o{:o}", foo) -> "0o33653337357"

// The same variable (foo) can be formatted differently depending on
// which argument type is used: X vs o vs unspecified.

// The formatting functionality is implemented via traits. There is one
// traid for each argument type. The most common formatting trait is Display.
// This handles cases where the argument type is left unspecified: '{}'

use std::fmt::{self, Formatter, Display}; //import self, Formatter and Display

struct City {
    name: &'static str,
    lat:  f32,
    lon:  f32,
}

impl Display for City {
    // 'f' is a buffer, this method must write the formatted string into it
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // 'write!' is like 'format!', but it will write the formatted string
        // into a buffer (the first argument, in this case f)
        write!(f, "{}: {:.3}°{} {:.3}°{}", self.name, self.lat, lat_c, self.lon, lon_c)
    }
}

// Generate the Debug format for structure Color
#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// ACTIVITY
// Add implementation for the fmt::Display trait for the Color structure.
// Output as:
// RGB (128, 255, 90) 0x80FF5A
// RGB (0, 3, 254) 0x0003FE
// RGB (0, 0, 0) 0x000000

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "RGB ({red}, {green}, {blue}) 0x{red:>02X}{green:>02X}{blue:>02X}",
                red=self.red, green=self.green, blue=self.blue)
        // 0x{:>02X}{:>02X}{:>02X} pad with 2 zeroes (:>02) and convert to hex (X)
    }
}

fn main() {
    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }

    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        println!("{:?}", *color)
    }

    // ACTIVITY
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        println!("{}", *color)
    }
}
