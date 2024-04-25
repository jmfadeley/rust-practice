/// Lessons "8.5.1.1 Destructuring Tuples"
/// https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring/destructure_tuple.html

// The syntax for destructuring tuples.
fn main() {
    let triple = (0, -2, 3);

    println!("Tell me about {:?}", triple);

    match triple {
        // Destructure the second and third elements
        (0, y, z) => println!("First is `0`, `y` is {:?} and `z` is {:?}", y, z),
        (1, ..) => println!("First is 1 and the rest don't matta."),
        (.., 2) => println!("Last is `2` don't care about rest."),
        (3, .., 4) => println!("First is `3`, last is `4`, don't care about others."),
        // `..` can be used to ignore the rest of the tuple
        _ => println!("Do not care!"),
        // `_` means do not bind to a variable.
    }
}