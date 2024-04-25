/// Lesson: "2.3 Arrays and Slices" 
/// Source: https://doc.rust-lang.org/rust-by-example/primitives/array.html


// An array naturally must be all the same T, but the length must also be known.
// This forms their signature [T; length]

use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("First element of slice: {}", slice[0]);
    println!("Last element of slice: {}", slice[slice.len()-1]);
    println!("Slice length: {}", slice.len());
}

fn main() {
    // Fixed-size array (type signature is superfluous).
    let xs: [i32; 5] = [1,2,3,4,5]; 
    println!("First element of xs array: {}", xs[0]);
    println!("Second element of xs array: {}", xs[1]);

    // Alternatively, you can initialize all elements with the same value.
    let ys: [i32; 50] = [0; 50];
    println!("First element of ys array: {}", ys[0]);
    println!("Fifth element of ys array: {}", ys[4]);

    // The len is the count, 1-based.
    println!("Length of xs: {}", xs.len()); // 5.

    // Arrays are stack allocated.
    println!("Array occupies {} delicious bytes.", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices.
    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);

    // Slices can point to a section of an array.
    // They are of the form [starting_index..ending_index].
    // `starting_index` is the first position in the slice.
    // `ending_index` is one more than the last position in the slice.
    println!("Borrow a section of the array as a slice.");
    analyze_slice(&xs[1..3]);

    // Let's see what an empty slice does.
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // Same but more verbose.

    // Arrays can be safely accessed using `.get` which returns an
    // `Option`. This can be matched as shown below, or used with 
    // `.expect()` if you would like hte program to exit with a nice message
    // instead of happily continue. Rust is so emotional.
    for i in 0..xs.len() +1 { // TOO FAR!
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Oh no! You asked for {} marbles and I don't have that many!", i),
        }
    }

    // Out of bound indexing on array causes compile time error.
    // println!("{}", xs[5]);
    // Out of bound indexing on slice causes runtime error.
    println!("{}", xs[..][5]);
}