// Rust provides type safety via static typing. Variable bindings can be type
// annotated when declared. However, in most cases, the compiler will be able to
// infer the type of the variable from the context, heavily reducing annotation burden.

// Values (like literals) can be bound to variables, using the 'let' binding.

fn main() {
    let an_integer = 1u32; // unsigned 32 bit int of value 1
    let a_boolean = true;  // boolean of value true
    let unit = ();          // unit of value ()

    // copy 'an_integer' into 'copied_integer'
    let copied_integer = an_integer;

    println!("Copied integer: {:?}", copied_integer);
    println!("An integer: {:?}", an_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // The compiler warns about unused variable bindings; these warnings can
    // be silenced by prefixing the variable name with an underscore
    let _unused_variable = 3u32;

    let _noisy_unused_variable = 2u32;
    // FIXME ^ Prefix with an underscore to suppress the warning
}
