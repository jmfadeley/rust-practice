/// Lessons "6.1 From and Into"
/// Sources: https://doc.rust-lang.org/rust-by-example/conversion.html

// Primitive types can be converted to each other through casting.

// Rust addresses conversion between custom types (e struct and enum) by the 
//use of traits. The generic conversions will use the `From` and `Into` traits.
// However there are more specific ones for the more common cases, in particular
// when converting to and from `String`s.

// `From` and `Into` are inherently linked. If you can A to be, then you should
// be able to B to A.

use std::convert::From;
use std::convert::Into;

//#[warn(dead_code)]
#[derive(Debug)]
struct Number {
    value: i32,
}

// Basically this is acting like a constructor.
// Takes in an i32 and returns a struct.
impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

// Into is reciprocal to `From`. This is commented out because
// `From` already creates an `Into` under the hood, and this 
// would cause a "conflicting implemention" error. To use this
// particular code, you want to comment out the `From` implementation
// above and all references to it.
// impl Into<Number> for i32 {
//     fn into(self) -> Number {
//         Number { value: self }
//     }
// }

// Using `Into` usually requires specification of hte type because
// the compiler cannot always determine this. But we get this functionality
// for free.

fn main() {
    // Here's from.
    let num = Number::from(30);
    println!("My number is {:?}", num);

    // This also works with the same `From` implementation
    // because, again, it happened under the hood.
    let int = 5;
    let num2: Number = int.into();
    println!("My new number is {:?}", num2);

    // let int = 6;
    // let num3 = int.into(); // Throws an error because no type declared.
    // println!("My new new number is {:?}", num3);

    // From is also quite common with `str` to String.
    let my_str = "Hello!";
    let my_string = String::from(my_str);
    println!("{}", my_string);
}