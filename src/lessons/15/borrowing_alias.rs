/// Lessons "15.3.1 Aliasing (Borrowing)"
/// https://doc.rust-lang.org/rust-by-example/scope/borrow/alias.html
 
/* Data can be immutably borrowed any number of times, but while immutably borrowed, the original data
cannot be mutably borrowed. On the other hand, only one mutable bororw is allowed at a time. The original
data can be borrowed again only after the mutable reference has been used for the last time. */
struct Point { x: i32, y: i32, z: i32} 

fn main() {
  let mut point = Point { x:0, y:0, z: 0};

  let borrowed_point = &point;
  let another_borrow = &point;

  println!("Point has coordinates: ({}, {}, {})", borrowed_point.x, borrowed_point.y, borrowed_point.z);

  // Error! Can't borrow point as mutable because it's currently borrowed as immutable.
  // let mutable_borrow = &mut point;
  // ^ Will error.

  // Borrowed values are used here again
  println!("Point has coordinates: ({}, {}, {})", borrowed_point.x, borrowed_point.y, borrowed_point.z);

  // The immutable references are no longer used for the rest of the code so it is possible to reborrow with a mutable reference.
  let mutable_borrow = &mut point;

  // change data via mutable ref:
  mutable_borrow.x = 5;
  mutable_borrow.y = 2;
  mutable_borrow.z = 1;

  // Error! Cannot borrow point as immutable because it's currently borrowed as mutable.
  // let y = &point.y;
  // ^ Remove to error out.

  // Also cannot print because println! takes an immutable reference
  // println!("Point Z coordinates is {}", point.z);

  // OK! Mutalbe references can be passed as immutable to println!
  println!("Point has coordinates: ({}, {}, {})", mutable_borrow.x, mutable_borrow.y, mutable_borrow.z);

  // The mutable ref is no longer used for the rest of the code so it is possible to reborrow
  let new_borrowed_point = &point;
  
  println!("Point has coordinates: ({}, {}, {})", new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z);
  
}