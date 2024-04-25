/// Lessons "8.2 Loop"
/// Sources: https://doc.rust-lang.org/rust-by-example/flow_control/loop.html

// There is a loop keyword. You can use `break` to escape or `continue` to skip
// the rest of this iteration and go onto the next.

fn main() {
    let mut count = 0u32;

    println!("Let's count until infinity or until I get bored.");
    loop {
        count += 1;
        if count == 3 {
            println!("three");
            continue;
        }
        println!("{}", count);

        if count == 5 {
            println!("I regret this. Breaking.");
            break;
        }
    }
}