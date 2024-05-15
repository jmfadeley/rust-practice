/// Lessons "9.3 Higher Order Functions"
/// https://doc.rust-lang.org/rust-by-example/fn/hof.html

// Rust offers HOFs that are a function that one or more functions to produce a more useful function.
// HOFs and lazy iterators are Rust's functional flavor.

fn is_odd(n: u32) -> bool {
  n % 2 == 1
}

fn main() {
  println!("Find the sum of all the nubmeres with odd squares under 1000");
  let upper = 1000;

  // Imperative approach
  let mut acc = 0;
  for n in 0.. {
    let n_squared = n^2;

    if n_squared >= upper {
      break;
    } else if is_odd(n_squared){
      acc+=n_squared;
    }
  }
  println!("Imperative result: {}", acc);

  // Functional
  let sum_of_squared_odd_numbers: u32 = 
    (0..).map(|n| n ^ 2)
         .take_while(|&n_squared| n_squared < upper)
         .filter(|&n_squared| is_odd(n_squared))
         .sum();
  println!("Functional result: {}", sum_of_squared_odd_numbers);

}