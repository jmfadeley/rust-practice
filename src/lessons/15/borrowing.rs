/// Lessons "15.3 Borrowing"
/// https://doc.rust-lang.org/rust-by-example/scope/borrow.html
 
/* Often we like to access data without taking ownership. To accomplish this, Rust uses a borrowing
mechanism. Instead o fpassing objects by value (T), objects can be passed by reference (&T).

The compiler statically guarantees via borrow checker that references always point to valid objects. 
That is, while references to an object exist, the object cannot be destroyed. */

// This funcfunctioniton takes ownership of a box and then destroys it violently.
fn eat_box_i32(boxed_i32: Box<i32>) {
  println!("Destroying box that contains {}", boxed_i32);
}

// this function borrows an i32
fn borrow_i32(borrowed_i32: &i32) {
  println!("This int is: {}", borrowed_i32);
}

fn main() {
  // Created a boxed i32 in the heap, and an i32 on the stack.
  // Remember: nunbers can have arbitrary underscores added for readability.
  // 5_i32 is the same as 5i32.
  let boxed_i32 = Box::new(5_i32);
  let stacked_i32 = 6_i32;

  // Borrow the contents of the box. Ownership is NOT taken.
  // so the contents can be borrowed again.
  borrow_i32(&boxed_i32);
  borrow_i32(&stacked_i32);

  {
    // Take a reference to the data contained inside the box.
    let _ref_to_i32: &i32 = &boxed_i32;

    // Error!
    // Can't destroy boxed_i32 while the inner value is borrowed later in scope
    // eat_box_i32(boxed_i32);
    // FIXME comment out this line

    // Attempt to borrow _ref_to_i32 after inner value is destroyed via eat_box_i32
    borrow_i32(_ref_to_i32);
  }

  eat_box_i32(boxed_i32);
}