/// Lessons "14.5 Multiple Bounds"
/// https://doc.rust-lang.org/rust-by-example/generics/multi_bounds.html

// Multiple bounds for a single type can be applied with a + sigil. Like normal, different
// types are separated with ,.

use std::fmt::{Debug, Display};

fn compare_prints<T: Debug + Display>(t: &T) {
  println!("Debug: `{:?}`", t);
  println!("Display: `{}`", t);
}

fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
  println!("t: `{:?}`", t);
  println!("u: `{:?}`", u);
}

fn main() {
  let string = "words";
  let array = [1, 2, 3];
  let vec = vec![1, 2, 3];

  compare_prints(&string);
  // compare_prints(&array);
  // Try uncommenting the above.

  compare_types(&array, &vec);
}