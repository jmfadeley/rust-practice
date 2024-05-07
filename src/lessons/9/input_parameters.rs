/// Lessons "9.2.2 As input parameters"
/// https://doc.rust-lang.org/rust-by-example/fn/closures/input_parameters.html

// Rust chooses how to capture variables on the fly mostly without type annotation
// But this isn't allowed when writing functions. When taking a closure as an input 
// parameter, the closure's complete type MUST be annotated using one of a few traits,
// as determined by what the closure does with the captured value. In order of decreasing
// restriction:

// `Fn`: Captures value by reference &T
// `FnMut`: Captures by mutable ref &mut T
// `FnOnce`: The closure uses the capture value by value (T)

// The compiler will opt for the least restrictive possible.
// IE a parameter annotated with `FnOnce` specifies that the closure MAY capture by &T,
// &mut T or T, but the compiler will ultimately choose based on how the capture variables are
// used in the closure.

// This is because if a move is possible, then any type of borrow should also be possible. Note
// that the reverse is not true. If the parameter is annoted as `Fn`, then capturing variables by 
// `&mut T` and `T` are not allowed. But `&T` is allowed. 

// A function which takes a closure as an argument and calls it.
// <F> denotes that F is a "generic type parameters."
fn apply<F>(f: F) where 
// The closure takes no input and returns nothing.
  F: FnOnce() {
    // TODO: Try changing this to Fn or FnMut.
    f();
  }

// A function which takes a closure and returns an `i32`.
fn apply_to_3<F>(f: F) -> i32 where// the closure takes an `i32` and returns one as well.
  F: Fn(i32) -> i32 {
    f(3)
  }

fn main() {
  use std::mem;

  let greeting = "hello";
  // A non-copy type.
  // `to_owned` creates owned data from a borrowed one.
  let mut farewell = "goodbye".to_owned();

  // Capture 2 variables `greeting` by reference and `farewell` by value.

  let diary = || {
    // `greeting` is by reference, and requires `Fn`
    println!("I said {}.", greeting);

    // Mutation forces `farewell` to be captured by
    // mutable ref. Now requires `FnMut`
    farewell.push_str("!!!");
    println!("Then I screamed {}.", farewell);
    println!("Now I can sleep. zzzz");

    // Manually calling drop forces `farewell` to be caputred by value.
    mem::drop(farewell);
  };

  // Call the function that requires the closure.
  apply(diary);

  // `double` satisfies the `apply_to_3`'s trait bound
  let double = |x| 2 * x;

  println!("3 doubled: {}", apply_to_3(double));
}