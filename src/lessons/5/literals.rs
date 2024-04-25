/// Lessons "5.2 Literals"
/// Sources: https://doc.rust-lang.org/rust-by-example/types/literals.html

// Numeric literals can be annotated by adding a suffix. ie 42i32.
// The defaults are i32 and f64, inferred.

// Using crate so I don't have to retype all that again and again.
use std::mem::*;

fn main() {
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Unsuffixed.
    let i = 1;
    let f = 1.0;

    // size_of_val to return the size of the variable in bytes.
    println!("size of `x` in bytes: {}", size_of_val(&x));
    println!("size of `y` in bytes: {}", size_of_val(&y));
    println!("size of `z` in bytes: {}", size_of_val(&z));
    println!("size of `i` in bytes: {}", size_of_val(&i));
    println!("size of `f` in bytes: {}", size_of_val(&f));
}