/// Lessons "14.4.1 Empty Bounds"
/// https://doc.rust-lang.org/rust-by-example/generics/bounds/testcase_empty.html

// A consequence of how bounds work is that even if a trait doesn't incldue any functionality, you
// can still use it as a bound. Eq and Copy are exampels of such traits from the std library.

struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

// These functions are only valid for types which implement these traits. The fact that the traits
// are empty is irrelevant.

fn red<T: Red>(_: &T) -> &'static str {"red"}
fn blue<T: Blue>(_: &T) -> &'static str {"blue"}

fn main() {
  let cardinal = Cardinal;
  let blue_jay = BlueJay;
  let _turkey = Turkey;

  // 'red()' won't work on a blue jay nor vice versa because of the bounds.
  println!("A cardinal is {}.", red(&cardinal));
  println!("A blue jay is {}.", blue(&blue_jay));
  // println!("A turkey is {}", red(&_turkey));
  // TODO: Try uncommenting the above line.
}