/// Lessons "15.3.3 The ref pattern (Borrowing)"
/// https://doc.rust-lang.org/rust-by-example/scope/borrow/ref.html

/* When doing pattern matching or destructing via the let binding, the ref keyword
can be used to take references to the fields of a struct/tuple. The example below shows 
a few instances where this can be useful: */

#[derive(Clone, Copy)]
struct Point { x: i32, y: i32}

fn main() {
  let c = 'Q';

  // A ref borrow on the left side of an assignment is equivalent to a & borrow on the right.
  let ref ref_c1 = c;
  let ref_c2 = &c;

  println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

  let point = Point { x: 0, y: 0 };

  // ref is also valid when destructing a struct.
  let _copy_of_x = {
    // ref_to_x is a reference to teh x field of point.
    let Point { x: ref ref_to_x, y: _ } = point;

    *ref_to_x;
  };

  // A mutable copy of point:
  let mut mutable_point = point;

  {
    // Ref can be paired with mut to take mutable references
    let Point { x:_, y: ref mut mut_ref_to_y }= mutable_point;

    // Mutate the y field of mutable_point via mutable reference.
    *mut_ref_to_y = 1;
  }

  println!("point is ({}, {})", point.x, point.y);
  println!("mutable_point is ({}, {})", mutable_point.x, mutable_point.y);

  // A mutable tuple that incldues a pointer:
  let mut mutable_tuple = (Box::new(5u32), 3u32);

  {
    //Destructure mutable_tuple to change the value of last:
    let (_, ref mut last) = mutable_tuple;
    *last = 2u32;
  }

  println!("tuple is {:?}", mutable_tuple);
}