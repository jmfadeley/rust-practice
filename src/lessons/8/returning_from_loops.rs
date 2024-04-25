/// Lessons "8.2.1 Returning from loops"
/// Sources: https://doc.rust-lang.org/rust-by-example/flow_control/return.html

// To try until you succeed and return something while quitting, just add it after the
// `break` like so.

fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}