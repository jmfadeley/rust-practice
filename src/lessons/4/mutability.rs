/// Lessons "2.1 Mutability"
/// Source: https://doc.rust-lang.org/rust-by-example/variable_bindings/mut.html

// `mut` can override the default immutability variable bindings.
fn main() {
    let _immutable_binding = 1; // i32!
    let mut mutable_binding = 1;

    mutable_binding += 1;

    println!("After mutation: {}" , mutable_binding);

    // This causes an error. Uncomment to see.
    // _immutable_binding += 1;
}