// A crate is a compulation unit in Rust. Whenever 'rustc some_file.rs' is called,
// 'some_file.rs' is treated as the crate file. If 'some_file.rs' has 'mod' declarations
// in it, then the contents of the module files will get merged with the crate file
// before running the compiler over it. In other words, modules DO NOT get compiled
// individually, only crates get compiled.

// A crate can be compiled into a binary or into a library. By default, rustc will
// produce a binary from a crate. This behavior can be overridden by passing the
// --crate-type flag to rustc.

// Let's create a library, and then see how to link it to another crate:
