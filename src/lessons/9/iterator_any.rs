/// Lessons "9.2.6.1 Iterator::any"
/// https://doc.rust-lang.org/rust-by-example/fn/closures/closure_examples/iter_any.html

// Iterator::any is a function which when passed an iterator, will return true if any element
// satisfies the predicate. Otherwise false. Behold!

pub trait Iterator {
  // The type being iterated over.
  type Item;

  // `any` takes `&mut self` meaning the caller may be borrowed and modified
  // but not consumed. Deliciously consumed.
  fn any<F>(&mut self, f: F) -> bool where
    // `FnMut` meaning any captured variable may at most be modified, not consumed
    // `Self::Item` states it takes arguments to the closure by value.
    F: FnMut(Self::Item) -> bool;
}

fn main() {
  let vec1 = vec![1, 2, 3];
  let vec2 = vec![4, 5, 6];

  println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
  println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

  // `iter()` only borrows `vec1` and its elements, so they can be used again.

  println!("vec1 len: {}", vec1.len());
  println!("first element of vec1 is: {}", vec1[0]);

  // `into_iter()` does move vec2 and its elements, to they CANNOT be used again
  // println!("First element of vec2 is: {}", vec2[0]);
  // println!("vec 2 len: {}", vec2.len());
  // TODO: Uncomment 2 lines above and watch the FIREWORKS!

  let array1 = [1, 2, 3];
  let array2 = [4, 5, 6];

  // `iter()` for arrays yields `&i32`
  println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
  println!("2 in array2: {}", IntoIterator::into_iter(array2).any(|x| x == 2));
}