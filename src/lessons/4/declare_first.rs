/// Lessons "4.3 Declare First"
/// Sources: https://doc.rust-lang.org/rust-by-example/variable_bindings/declare.html

// You can declare a variable binding first and initialize it later. This is seldom
// used as it may lead to the use of uninitialized variables, which the compiler forbids. 

fn main() {
    let a_binding;
    {
        let x = 2;
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    // This will throw an error for being undefined.
    // println!("another binding: {}", another_binding);

    another_binding = 1;

    println!("another binding: {}", another_binding);
}