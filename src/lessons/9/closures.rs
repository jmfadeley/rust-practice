/// Lessons "9.2 Closures"
/// https://doc.rust-lang.org/rust-by-example/fn/closures.html
 
// Closures are functions that capture the enclosing environment.  
// |val| val + x would capture the x variable.
 
// Closures are useful for on the fly, and they're called like a
// function. But the input and return types can be inferred and
// input variable names must be specified.

// Other characteristics:
// || instead of () around input variables.
// Optional body delimitation ({}) for a single line expression (else mandatory)
// Ability to capture the outer env variables.

fn main() {
  let outer_var = 42;

  // Reg functions can't refer to variables in the enclosing environment, ie:
  // fn function(i: i32) -> i32 { i + outer_var } 
  // the above would throw an error.

  // Closures are anonymous and I'll bind them to references.
  // Annotation is identical to function annotation but is optional.
  // as are the {} wrapping the body/block.
  let closure_annotated = |i: i32| -> i32 { i + outer_var };
  let closure_inferred = |i| i + outer_var;

  println!("closure_annotated: {}", closure_annotated(1));
  println!("closure_inferred: {}", closure_inferred(1));

  // You can also take in no arguments and return an i32 through
  // inferrence.
  let one = || 1;
  println!("closure returning one: {}", one());
}