/// Lessons "8.3 While"
/// Sources: https://doc.rust-lang.org/rust-by-example/flow_control/while.html
 
// You can use `while` to run while a condition is true.

fn main() {
    let mut n = 1;

    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);3
        }

        n += 1;
    }
}