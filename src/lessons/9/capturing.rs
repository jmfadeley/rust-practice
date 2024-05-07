/// Lessons "9.2.1 Capturing"
/// https://doc.rust-lang.org/rust-by-example/fn/closures/capture.html

// Closures will do what the functionality requires to make them work. 
// They can capture variables"

// by reference: %T
// by mutable reference: &mut T
// by value: T

// Reference is prefered but they go lower in this list when required.

fn main() {
  use std::mem;

  let color = String::from("Green");

  // Reference here, storing hte borrow and closure in the print variable.
  // It will remained borrowed until the last time print is used;
  let print = || println!("`color`: {}", color);

  // Call the closure using the borrow.
  print();

  // `color` can be borrowed immutably again, because the closure only holds
  // an immutable reference to `color`.
  let _reborrow = &color;
  print();

  // Again for fun.
  print();

  // A move or reborrow is allowed after the final use of `print`
  let _color_moved = color;

  let mut count = 0;
  // A closure to increment `count` could take either `&mut count` or `count`,
  // but `&mut count` is less restrictive so it takes that. Immeidately borrows `count`.

  // A `mut` is required on `inc` because a `&mut` is stored inside. Thus, calling
  // the closure mutates `count` which requires a `mut`.

  let mut inc = || {
    count += 1;
    println!("`count`: {}", count);
  };

  // Call the closure using a mutable borrow.
  inc();

  // The closure still mutably borrows `count` because it is called later.
  // At attempt to reborrow will lead to an error. 
  // let _reborrow = &count;
  // ^ this would error if the below is uncommented.
  inc();

  // The closure no longer needs to borrow `&mut count`. Therefore
  // it can be reborrowed without error.
  let _count_reborrowed = &mut count;

  // A non-copy type.
  let movable = Box::new(3);

  // `mem::drop` required `T` so this must take by value. A copy type
  // would copy into the closure leaving the original untouched.
  // A non-copy must move and so `lovable` immeidately moves into
  // the closure.
  let consume = || {
    println!("`movable`: {:?}", movable);
    mem::drop(movable);
  };

  // This consume the variable so it can only be called once.
  consume();
  // consume();
  // ^ This will error.

  // There is also the `move` keyword, which when used before the closure's pipes
  // forces closure to take ownership of captured variables.

  // `Vec` has non-copy semantics.
  let haystack = vec![1,2,3];

  let contains = move |needle| haystack.contains(needle);
  println!("{}", contains(&1));
  println!("{}", contains(&4));

  // println!("There're {} elements in vec", haystack.len());
  // ^ causes error because the borrow checker doesn't allow re-using a
  // moved variable.

  // However, if you take out `move`, the signature will cause closure to borrow haystack
  // immutably, which would allow the above to work.
}