// There are three types of structures ("structs") that can be created
// using the struct keyword:

// Tuple structs, which are, basically, named tuples.
// The classic C structs
// Unit structs, which are field-less, are useful for generics.

// A unit struct
struct Nil;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug, Copy, Clone)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
#[derive(Copy, Clone)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

// ACTIVITY
// Add a function rect_area which calculates the area of a rectangle (try using nested
// destructuring).

fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle { p1: _p1, p2: _p2 } = rect; // if we intend to use rect
    // outside this function we need to implement the Copy and Clone traits
    // for Rectangle. Which means it needs to also be implemented for Point

    // Calculate the length and width
    let length = if _p1.x >= _p2.x { _p1.x - _p2.x } else { _p2.x - _p1.x };
    let width  = if _p1.y >= _p2.y { _p1.y - _p2.y } else { _p2.y - _p1.y };

    length * width
}


// ACTIVITY
// Add a function square which takes a Point and a f32 as arguments, and returns
// a Rectangle with its lower left corner on the point, and a width and height
// corresponding to the f32.

fn square(start_point: Point, length: f32) -> Rectangle {
    let Point { x, y } = start_point;
    Rectangle { p1: start_point, p2: Point { x: x + length, y: y + length } }
}

fn main() {
    // Instantiate a `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };
    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);
    // Destructure the point using a `let` binding
    let Point { x: my_x, y: my_y } = point;
    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };
    // Instantiate a unit struct
    let _nil = Nil;
    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);
    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);
    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);

    // ACTIVITY - area
    let rectangle = Rectangle{ p1: Point{ x: 0.0, y: 0.0 }, p2: Point{ x: 5.0, y: 5.0 } };
    println!("Rectangle has an area of: {}", rect_area(rectangle));

    // ACTIVITY - square
    let rectangle2 = square(Point { x: 1.0, y: 2.0 }, 10.0);
    println!("Square from {:?} to {:?} with area {}",
            rectangle2.p1, rectangle2.p2, rect_area(rectangle2))
}
