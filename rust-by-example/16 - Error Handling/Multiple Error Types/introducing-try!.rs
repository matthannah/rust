// Sometimes we just want the simplicity of unwrap without the possibility of a panic. Until now,
// unwrap has forced us to nest deeper and deeper when what we really wanted was to get the
// variable out. This is exactly the purpose of try!.

// Upon finding an Err, there are two valid actions to take:

// panic! which we already decided to try to avoid if possible
// return because an Err means it cannot be handled

// try! is almost exactly equivalent to an unwrap which returns instead of panics on Errs.
// Let's see how we can simplify the earlier example that used combinators:

// Use `String` as our error type
type Result<T> = std::result::Result<T, String>;

fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = try!(vec.first()
        .ok_or("Please use a vector with at least one element.".to_owned()));

    let value = try!(first.parse::<i32>()
        .map_err(|e| e.to_string()));

    Ok(2 * value)
}

fn print(result: Result<i32>) {
    match result {
        Ok(n)  => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(empty));
    print(double_first(strings));
}

// Note that up until now, we've been using Strings as errors. However, they are somewhat limiting
// as an error type. In the next section, we'll learn how to make more structured and informative
// errors by defining their types.
