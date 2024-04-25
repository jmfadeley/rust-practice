/// Lessons "8.7 Let-else"
/// Sources: https://doc.rust-lang.org/rust-by-example/flow_control/let_else.html

// With let-else, a refutable pattern can match and bind variables in the surrounding
// scope like a normal let, or else diverse (break, return panic!) when the pattern
// doesn't match.

use std::str::FromStr;

// This is what let-else lets you do.
fn get_count_item(s: &str) -> (u64, &str) {
  let mut it = s.split(' ');
  let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
    panic!("Can't segment count item pair: '{s}'");
  };
  let Ok(count) = u64::from_str(count_str) else {
    panic!("Can't parse integer: '{count_str}'");
  };
  (count, item)
}

// fn get_count_item2(s: &str) -> (u64, &str) {
//   let mut it = s.split(' ');
//   let (count_str, item) = match (it.next(), it.next()) {
//     (Some(count_str), Some(item)) => (count_str, item),
//     _ => panic!("Can't segment count item pair: '{s}'"),
//     };
//   let count = if let Ok(count) = u64::from_str(count_str) {
//       count
//   } else {
//       panic!("Can't parse integer: '{count_str}'");
//   };
// }


fn main() {
  assert_eq!(get_count_item("3 chairs"), (3, "chairs"));
// Otherwise, you'd get a bunch of repetition and an outer let:
  // assert_eq!(get_count_item2("3 chairs"), (3, "chairs"));
}