use std::error::Error;
use std::io::BufRead;

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.capacity() != 2 {
        exit("Please enter argument to program: /path/to/file.txt");
    }

    let path: &std::path::Path = std::path::Path::new(&args[1]);
    let display = path.display();
    // Validation
    if !path.exists() {
        exit("Path entered does not point to an existing file");
    }
    if !path.is_file() {
        exit("Path entered does not point to a regular file");
    }

    // Read file
    let f = match std::fs::File::open(&path) {
        Err(err) => panic!("couldn't open {}: {}", display, err.description()),
        Ok(file) => file,
    };
    let file = std::io::BufReader::new(&f);

    let mut line_count: usize = 0;
    for _ in file.lines() {
        line_count += 1;
    }

    println!("{} lines in {}", line_count, display);
}

fn exit(message: &'static str) {
    println!("{}", message);
    std::process::exit(0);
}
