/// Lessons "9.2.6.2 Searching through iterators"
/// https://doc.rust-lang.org/rust-by-example/fn/closures/closure_examples/iter_find.html
// Iterator::find returns the first value that satisfies the condition. Otherwise returns none.

pub trait Iterator {
  type Item;

  fn find<P>(&mut self, predicate: P) -> Option<Self::Item> where
  // `FnMut` meaning any capture dvariable may at most be modified but not consumed
  P: FnMut(&Self::Item) -> bool;
}

fn main() {
  let vec1 = vec![1, 2, 3];
  let vec2 = vec![4, 5, 6];

  let mut iter = vec1.iter();
  let mut into_iter = vec2.into_iter();

  println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2));
  println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x == 2));

  let array1 = [1, 2, 3];
  let array2 = [4, 5, 6];

  println!("Find 2 in array1: {:?}", array1.iter().find(|&&x| x ==2));
  println!("Find 2 in array2: {:?}", IntoIterator::into_iter(array2).find(|&x| x ==2));

  // Find gives you the reference to an item, but if you want the index, use position

  let vec3 = vec![1, 9, 3, 3, 13, 2];

  // `iter()` for vecs yields &i32 and position does not take a reference but we've got to destruct &i32 into i32"
  let index_of_first_even_number = vec3.iter().position(|&x| x % 2 == 0);
  assert_eq!(index_of_first_even_number, Some(5));

  // MEANWHILE
  let index_of_first_negative_number = IntoIterator::into_iter(vec3).position(|x| x < 0);
  assert_eq!(index_of_first_negative_number, None);
}