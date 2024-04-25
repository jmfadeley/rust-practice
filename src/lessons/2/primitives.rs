/// Lesson: "2. Primitives" 
/// Source: https://doc.rust-lang.org/rust-by-example/hello

// Scalar type primitives.
// Signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
// Unsigned integers u8, u16, u32, u64, u128, usize (pointer)
// Floating point f32 and f64
// char Unicdoe scalar values like 'a', 4 bytes apiece.
// bool which is true or false... can it be undefined or does it default to false? Let's find out!
// (), which is a unit type for an empty tuple. It is not a compound until it contains multiple values.

// Compound types:
// Arrays, like [1, 2, 3]
// Tuples like (1, true, 'b')
// Guess Strings are objects?

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    // Variable can always be type annotated.
    let logical: bool = true;

    // Numbers can always been annotated by regular or suffix annotation.
    let a_float: f64 = 1.0;
    let an_integer = 5i32; // Suffix for true suffering.

    // Or just use the defaults:
    let default_float = 3.0; // f64.
    let default_integer = 7; // i32;

    // Type can also be inferred from context.
    let mut inferred_type = 12; // Might start as i32, but next line infers it as i64.
    print_type_of(&inferred_type); // Returns i64, but if you comment out the following line, it's i32.
    inferred_type = 12342345211i64; 
    print_type_of(&inferred_type); 

    // It looks like you must use suffix over regular annotation. The below doesn't work.
    // inferred_type: i8 = 4;

    let mut mutable = 12;
    mutable = 21;

    // This will throw an error because variables cannot be changed.
    // mutable = true;

    // However, you can use shaddowing to overwrite. See less 4.2 for more.
    let mutable = true;
}