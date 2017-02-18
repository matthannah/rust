// A tuple is a collection of values of different types. Tuples are contructed
// using parentheses (), and each tuple itself is a value with type signature
// (T1, T2, ...), where T1, T1 are the types of its members. Functions can use
// tuples to return multiple values, as tuples can hold any number of values.

// Tuples can be used as function arguments and as return values.
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // 'let' can be used to bind the members of a tuple to variables
    let (integer, boolean) = pair; // pair was passed into the function

    (boolean, integer) // using the bindings made by 'let' we can return the reverse!
}

// The following struct is for the ACTIVITY
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

// ACTIVITY
// Recap: Add the fmt::Display trait to the Matrix struct in the above example,
// so that if you switch from printing the debug format {:?} to the display format
// {}, you see the following output:

// ( 1.1 1.2 )
// ( 2.1 2.2 )

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

// ACTIVITY
// Add a transpose function using the reverse function as a template,
// which accepts a matrix as an argument, and returns a matrix in which two elements
// have been swapped. For example:

// println!("Matrix:\n{}", matrix);
// println!("Transpose:\n{}", transpose(matrix));

// results in the output:

// Matrix:
// ( 1.1 1.2 )
// ( 2.1 2.2 )

// Transpose:
// ( 1.1 2.1 )
// ( 1.2 2.2 )

fn transpose(matrix: Matrix) -> Matrix {
    let Matrix(a, b, c, d) = matrix;
    Matrix(a, c, b, d)
}

fn main() {
    // A tuple with a bunch of different types
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                     -1i8, -2i16, -3i32, -4i64,
                     0.1f32, 0.2f64,
                     'a', true);

    // Values can be extracted from the tuple using tuple indexing
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // Tuples can be tuple members
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    let pair = (1, true);
    println!("pair is {:?}", pair);

    println!("the reversed pair is {:?}", reverse(pair));

    // To create one element tuples, the comma is required to tell them apart
    // from a literal surrounded by parentheses
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    // tuples can be destructured to create bindings
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple; // this is real cool
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);

    println!("{:?}", reverse((2, true))); // will print (true, 2)

    // ACTIVITY - Display
    println!("{}", matrix);

    // ACTIVITY - Transpose
    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix))
}
