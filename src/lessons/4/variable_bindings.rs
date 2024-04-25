/// Lessons "2. Variable Bindings"
/// Sources: https://doc.rust-lang.org/rust-by-example/variable_bindings.html

// Rust uses static typing for type safety. 

fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // Copy `an_integer` into `copied_integer`
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // The compiler warns about unused variable bindings, but adding an 
    // underscore silences this. Note that these warnings stop the compiling.
    let _unused_variable = 3u32;

    // Uncomment...
    // let noisy_unused_variable = 2u32;
}