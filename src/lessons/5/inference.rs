/// Lessons "5.3 Inference"
/// Sources: https://doc.rust-lang.org/rust-by-example/types/inference.html

// The type inference is smart enough to check for how the variable is used after infered 
// type. Observe.
fn main() {
    // Annotation means it's type `u8`
    let elem = 5u8;

    // Empty vector (growable array).
    let mut vec = Vec::new();

    // The compiler doesn't know right now what type `vec` will be, so
    // it's akin to `Vec<_>` until something is given.

    // Insertion.
    vec.push(elem);

    // Compiler now figures that `vec` is supposed to be a Vector of `u8`s
    println!("{:?}", vec);
    // To have more fun, try commenting out `vec.push(elem)` to get a type warning.
}