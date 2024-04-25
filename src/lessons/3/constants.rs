/// Lesson: "3.3 constants" 
/// Source: https://doc.rust-lang.org/rust-by-example/custom_types/constants.html

// There are two types of constants.
// Static: Possibly mutable with `'static` lifetime. Lifetime is inferred and does not
// have to be specified. Accessing or modifying a mutable static variable is unsafe.
// Const: Unchangeable, only shadowable.
static LANGUAGE: &str = "Rust"; 
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > THRESHOLD
}

fn main() {
    let n = 16; // i32

    // Access constant in the main thread:
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // This will throw an error.
    // THRESHOLD = 5;
}