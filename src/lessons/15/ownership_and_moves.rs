/// Lessons "15.2 Ownership and Moves"
/// https://doc.rust-lang.org/rust-by-example/scope/move.html
 
/* Because variables are in charge of freeing hteir own resources, resources can only have one owner.
This prevents resources from being freed more than once. Note that not all variables own resources (references).

When doing assignments ie let x = 7 or passing funciton arguments by value (foo(x)) the ownership of the resources
is transferred. In Rust-speak, this is known as a move.

After moving resources, the previous owner can no longer be used. This avoids creating dangling pointers.
*/

fn destroy_box(c: Box<i32>) {
  println!("Destroying the box that contains {}", c);

  // Goodbye c, and hello memory reclaimed.
}

fn main() {
  // _Stack_ allocated integer
  let x = 5u32;

  // Copy x into y, no resources are moved.
  let y = x; // Think copying of primitives in Java. But remember this is immutable.

  println!("x is {}, and y is {}", x, y);
  // Likewise...
  let mut z = y; // Now THIS is mutable!
  println!("y is {}, and z is {}", y, z);

  z = 7u32;
  println!("y is {}, and z is {}", y, z);

  // a is a pointer ot a heap allocated integer.
  let a = Box::new(5i32);

  println!("a container: {}", a);

  // Move a into b..
  let b = a;
  // The pointer address of a is copied but NOT the data, into b. Both are now poiners to the same
  // heap allocated data, but b now owns it.

  // Because b now owns it, this will fail.
  // println("a container: {}", a);
  // ^ Fails.
  println!("b container: {}", b);

  // This function takes ownership of the heap allocated memory from b.
  destroy_box(b);

  // Since the heap memory is now freed at this point, this action would result in dereferencing freed memory, but it's forbidden by the compiler

  // println("b container: {}", b);
  // ^ Fails.
}