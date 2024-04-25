/// Lessons "6.3 To and From Strings"
/// Sources: https://doc.rust-lang.org/rust-by-example/conversion/string.html

// To convert any typ einto a `String` is as simple as implementing the `ToString` 
// trait for the type. Like Java. Rather than doing so directly, you should implement
// the `fmt::Display` trait which automagically (yes magic) provides `ToString` and
// allows printing the type to `print!`

use std::fmt;

struct Circle {
    radius: i32
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string()); // Witchcraft!

    // You can also `parse` them from String to number, or other types.
    // You can use inferrence or the so called 'turbofish' syntax. 

    // The important thing is that `FromStr` is implemented for that type,
    // which it seems to be for the types of the standard library.

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {}", sum); 
}