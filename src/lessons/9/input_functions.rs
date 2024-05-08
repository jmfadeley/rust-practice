/// Lessons "9.2.4 Input Functions"
/// https://doc.rust-lang.org/rust-by-example/fn/closures/input_functions.html

// Both closures and functions can be used as arguments. Any function that takes a closure
// as an argument can be called with a function instead.

fn call_me<F: Fn()>(f: F) {
  f();
}

// Define a wrapper function satisfying the Fn bound

fn function() {
  println!("I'm a function watch me roar.");
}

fn main() {
  // define a closure satisfying the Fn bound
  let closure = || println!("I'm a closure boooo.");

  call_me(closure);
  call_me(function);
}