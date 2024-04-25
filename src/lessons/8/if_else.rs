/// Lessons "8.1 If/Else"
/// Sources: https://doc.rust-lang.org/rust-by-example/flow_control/if_else.html

// If/Else is pretty much the same as anywhere else. However, an if/else block must return
// the same type at the end of all branches. You can't return a String in one and a number 
// from another.

fn main() {
    let n = 5;

    if n < 0 { // Parantheses around the conditions gets a warning.
        print!("{} is negative", n); // print! because we're going to append more.
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n = 
        if n < 10 && n >-10 {
            println!(", and is a small number, increase ten-fold");
            10 * n // Comes back as i32.
        } else {
            println!(", and is a big number, halve the number");
            10 / 2
        }; // Remember! ; for block assignments of lets.

    println!("{} -> {}", n, big_n);

    // Let's try a inferred assignment! ... And this will throw an error.
    // let what = 
    //     if n > 0 {
    //         "String"
    //     } else {
    //         8
    //     };
    // println!("{}", what);
}