/// Lesson: "3.2.2 C-Like" 
/// Source: https://doc.rust-lang.org/rust-by-example/custom_types/enum/c_like.html


#![allow(dead_code)]

enum Number {
    Zero, 
    One,
    Two
}

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    println!("zero is {}", Number::Zero as i32);
    println!("one is P{}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violents are #{:06x}", Color::Blue as i32);
}