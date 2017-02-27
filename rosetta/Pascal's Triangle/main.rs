#![allow(dead_code)]

const ROWS: usize = 10;
const PADDING: usize = 3;
const WIDTH: usize = ROWS + (PADDING * (ROWS - 1));

struct PascalsTriangle ([[usize; ROWS]; ROWS]);

impl PascalsTriangle {
    fn new() -> PascalsTriangle {
        let mut array = [[0; ROWS]; ROWS];
        array[0][0] = 1;

        for row in 1 .. ROWS { // start at row 1
            for col in 0 .. ROWS {
                if col > row { break; }
                let mut value = array[row -1][col];

                if col != 0 {
                    value += array[row - 1][col - 1]
                }

                array[row][col] = value;
            }
        }
        PascalsTriangle(array)
    }
}

// Printing the triangle
impl std::fmt::Display for PascalsTriangle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in 0 .. ROWS {
            for col in 0 .. ROWS {
                if col == 0 {
                    pad((WIDTH - (row * PADDING) - (row + 1)) / 2);
                } else {
                    pad(PADDING);
                }
                if self.0[row][col] != 0 {
                    try!(write!(f, "{}", self.0[row][col]));
                }
            }
            try!(writeln!(f, ""));
        }
        write!(f, "")
    }
}

fn pad(count: usize) {
    for _ in 0 .. count { print!(" "); }
}

fn main() {
    let triangle = PascalsTriangle::new();
    println!("{}", triangle)
}
