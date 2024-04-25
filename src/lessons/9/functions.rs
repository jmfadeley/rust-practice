/// Lessons "9 Functions"
/// https://doc.rust-lang.org/rust-by-example/fn.html

// Functions are declare with fn. Arguments are type annoted like variables
// and if the function returns a value, the return type must be specified 
// after an arrow ->
// The final expression will be used to return a value. Or you can use a return 
// statement, even inside another block (say an if block).

fn main() {
  fizzbuzz_to(100);
}

// Returns a bool
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
  if rhs == 0 {
    return false;
  }
  lhs % rhs == 0
}

// If a function doesn't return a value, it returns a () tuple type.
fn fizzbuzz(n: u32) -> () {
  if is_divisible_by(n, 15) {
    println!("fizzbuzz");
  } else if is_divisible_by(n, 3) {
    println!("fizz");
  } else if is_divisible_by(n, 5) {
    println!("buzz");
  } else {
    println!("I'm bored.");
  }
}

// Or just omit that works fine.
fn fizzbuzz_to(n: u32) {
  for n in 1..=n {
    fizzbuzz(n);
  }
}