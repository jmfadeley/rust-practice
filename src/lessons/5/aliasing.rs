/// Lessons "5.4 Aliasing"
/// Sources: https://doc.rust-lang.org/rust-by-example/types/alias.html

// The `type` statement can give a new name to an existing type. 
// Just use an UpperCamelCase name or the compiler will raise warnings.
// The exception to the warning is when using `usize`, `f32`, etc. 

type NanoSecond = u64;
type Inch = u64;
type U64 = u64;

fn main() {
    // So `NanoSecond` = `Inch` = `U62` = `u64`
    let nanoseconds: NanoSecond = 5 as U64;
    let inches: Inch = 2 as U64;
    let burp_length: NanoSecond = 4;

    // However type aliases provide no extra type safety because they are
    // not new types.
    println!("{} seconds + {} inches = {} unit?",
        nanoseconds,
        inches,
        nanoseconds + inches);

    println!("LOL. {}.", burp_length);
}