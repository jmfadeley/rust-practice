/// Lessons "8.5.1.2 Destructuring Arrays/Slices"
/// https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring/destructure_slice.html

// The syntax for destructuring Arrays or slices..
fn main() {
    let array = [4, -2, 6];

    match array {
        [0, second, third] => // Direct binding to positions.
            println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),
        [1, _, third] => // _ will ignore a value.
            println!("array[0] = 1, array[2] = {} and array[1] was ignored", third),
        [-1, second, ..] => println!( // Bind some, ignore rest. To get the third of four+, [-1, _, third, ..]
            "array[0] = -1, array[1] = {}, and all the other ones were ignored", second),
        // This would not compile...
        // [-1, second] => ...
        // This stores the remainder in another array/slice but {:?} because you don't know type.
        [3, second, tail @ ..] => println!( 
            "array[0] = 3, array[1] = {} and the other elements were {:?}", second, tail 
        ),
        // Finally you can combo this for the first, last and everything inbetween in a single array.
        [first, middle @ .., last] => println!(
            "array[0] = {}, middle = {:?}, array[2] = {}",
            first, middle, last
        ),
    }
}