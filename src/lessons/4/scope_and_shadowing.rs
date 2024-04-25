/// Lessons "4.2 Scope and Shadowing"
/// Sources: https://doc.rust-lang.org/stable/rust-by-example/variable_bindings/scope.html

fn main() {
    // Normally variables are immutable by default. The following would throw an error at compile time:
    // an_integer = 7;
    // println!("{}", an_integer);

    // mut lets you change the variable's value.
    let mut mutable = 12; // Inferred to be i32, as above.
    println!("{}",  mutable);
    mutable = 21;
    println!("{}",  mutable);

    // However, mut cannot change the variable type. The below throws a compile error.
    // mutable = true;
    // println!("{}",  mutable);

    // Shadowing, even in a local block is available.
    let mutable = true;
    println!("{}",  mutable);

    // What is shadowing? Usually it allows for casting of local variables with the same names as higher-tier scope.
    // For example, this is a block with a variable limited scope.

    let long_lived_binding = 1; // Remember, i32.
    {
        let short_lived_binding = 2.0; // Remember, f64.
        println!("Short lived the inner binding of {}!", short_lived_binding);
    }

    // This would throw a compile error.
    // println!("Short lived the outer binding of {}!", short_lived_binding);
    println!("Short lived the outer binding of {}!", long_lived_binding);

    // This is an example of same-level shadow binding.
    let shadowed_binding = 1;
    {
        println!("Before being shadowed: {}", shadowed_binding);
        // This throws an error if uncommented...
        // shadowed_binding = "abc";
        // But this is fine, although it only lives for this block.
        let shadowed_binding = "abc";
        println!("Shadowed inner block: {}", shadowed_binding); // abc
    }
    println!("Outer block: {}", shadowed_binding); // Back to 1.

    let shadowed_binding = 3.0;
    println!("Shadowed outer block: {}", shadowed_binding); // 3.

    // Is mut maintained between shadow binding?
    let mut whatever = 1; // If not used, these will throw warnings so println...
    println!("{}", whatever);
    whatever = 2;
    println!("{}", whatever);
    let whatever = "xyz";
    println!("{}", whatever);
    // whatever = "abc";

    // Nope! Compule error on "abc" cast. "mut" is not maintained.
}
