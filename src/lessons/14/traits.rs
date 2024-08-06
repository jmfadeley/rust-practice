/// Lessons "14.3 Traits"
/// https://doc.rust-lang.org/rust-by-example/generics/gen_trait.html

// Traits can be generic. Here we define one which reimplements the Drop trait as a generic method to drop
// itself and an input.

struct Empty;
struct Null;

// Trait generic over T
trait DoubleDrop<T> {
  // Define a method on a caller type which takes an additional single paramter and does nothing with it.
  fn double_drop(self, _: T);
}

// Implment DoubleDrop<T> for any generic parameter T and a caller U.
impl<T, U> DoubleDrop<T> for U {
  // This method takes ownership of both passed arguments, deallocating both.
  fn double_drop(self, _: T) {}
}

fn main() {
  let empty = Empty;
  let null = Null;

  // Deallocated empty and null:
  empty.double_drop(null);

  // empty;
  // null;
  // Try uncommenting these.
}
