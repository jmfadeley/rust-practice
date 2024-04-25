/// Lessons "8.2.1 Nesting and labels"
/// Sources: https://doc.rust-lang.org/rust-by-example/flow_control/nested.html

// It's possible to `break` or `continue` outer loops when doing nested loops. 
// Use `'label` and pass this label to the `break` or `continue` statement.

#[allow(unreachable_code, unused_labels)]

fn main() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // This would break only the inner loop.
            // break;

            // This breaks the outer loop.
            break 'outer;
        }
        println!("You will never reach this line! NEVER!");
    }
    println!("I have escaped the outer loop!");
}