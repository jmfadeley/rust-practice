/// Lessons "8.5 Match"
/// Sources: https://doc.rust-lang.org/rust-by-example/flow_control/match.html

// Match is basically C's switch. However, there's a bunch of bells and whistles in the Destructuring lesson next.

fn main() {
    let number = 20; // i32.
    // Try different number values.

    match number {
        // Single match.
        1 => println!("One is such a lonely number."), // No semicolon!
        2 | 3 | 5 | 7 | 11 => println!("This is prime!"),
        // Try adding 13 to this list. Result: Prime supercedes teen.
        13..=19 => println!("A mere teen..."),
        // All other cases.
        _ => println!("Ain't special."),
        // Try commenting out this default catch.
    }

    let boolean = true;
    // Match is an expression too. Observe!
    let binary = match boolean {
        // The arms of a match must cover all possible values!
        false => 0,
        true => 1, // Try commenting this out.
    };

    println!("{} -> {}", boolean, binary);
}