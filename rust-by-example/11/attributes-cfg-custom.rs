// Some conditionals like 'target_os' are implicitly provided by 'rustc', but custom
// conditionals must be passed to 'rustc' using the --cfg flag.

fn main() {
    #[cfg(some_condition)]
    const CONDITION: &'static str = "conditional met";

    #[cfg(not(some_condition))]
    const CONDITION: &'static str = "conditional not met";

    println!("{}", CONDITION);
}

// Without the custom cfg flag:

// $ rustc attributes-cfg-custom.rs
// ./attributes-cfg-custom.rs

// With the custom cfg flag:

// $ rustc --cfg some_condition attributes-cfg-custom.rs
// ./attributes-cfg-custom.rs
// condition met!
