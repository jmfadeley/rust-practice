/// Lessons "15.4.1 Explicit annotation"
/// https://doc.rust-lang.org/rust-by-example/scope/lifetime/explicit.html

/* The borrow check uses explicit lifetime annotations to determine how long references should be valid.
In cases where lifetimes are not elided (different from lifetime), Rust requires explicit annotations to
determine what the lifetime of a reference should be. The syntax for explicitly annotating a lifetime uses
an apostrophe character as follows:

foo<'a>
// foo has a lifetime parameter 'a

Similar to closures, using lfietimes requires generics. Also this lifetime syntax indicates that the life 
of foo may not seed that of 'a. Explicit annotation of a type has the form &'a T where 'a has already been 
introduced.

In cases with multiple lifetimes, the syntax is similar:

foo<'a, 'b>

In this case the lifetime of foo cannot exceed that of either 'a or 'b.
*/

fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
  println!("x is {}, y is {}", x, y);
}

// A function which takes no arguments but has a lifetime parameter 'a
fn failed_borrow<'a>() {
  let _x = 12;

  // Error: +X does not live long enough
  let _y: &'a i32 = &_x;

  // Attemping to use the lifetime 'a as an explicit type annotation inside the 
  // function will fail because the lifetime of &_x is shorter than that of _y. A
  // shorter lifespan cannot be coerced into a longer one.
}

fn main() {
  // Create variables to be borrowed below.
  let (four, nine) = (4,9);

  // Borrows & of both variables are passed into the function.
  print_refs(&four, &nine);

  // Any input which is borrowed must outlive the borrower. 
  // In other words the lifetime of four and nine must be longer than that of print_ref.

  failed_borrow();
  // failed_borrow contain no ref to force 'a to be longer than the lifetime of the 
  // function but 'a is longer. Because the lifetime is never constrained, it defaults to
  // 'static.
}