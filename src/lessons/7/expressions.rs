/// Lessons "7.0 Expressions"
/// Sources: https://doc.rust-lang.org/rust-by-example/expression.html

fn main() {
    // Blah blah blah. Expressions are usually variable bindings and statements that end
    // in ;. 
    let x = 5;
    x; 
    x + 1;
    15;

    // However blocks can be expressions too and even assigned as values. The last expression
    // would be returned. But if it ends with a semicolon, the return value is (), an empty tuple.

    let a = 5u32;

    let b = {
        let a_squared = a * a;
        let a_cubed = a_squared * x;
        a_cubed + a_squared + a // Notice the lack of a ;
    }; // Notice the ; here.

    let c = {
        2 * a; // Notice this one gets a semicolon...
    };

    println!("a is {}", a);
    println!("b is {}", b);
    println!("c is {:?}", c); // Tuple doesn't have display.
}