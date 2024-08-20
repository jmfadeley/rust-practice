/// Lessons "14.6 Where Clauses"
/// https://doc.rust-lang.org/rust-by-example/generics/where.html

// A bound can also be expressed using a where clause immediately before the opening {, rather than
// at the type's first mention. wher clauses can apply bounds to arbitrary types too, and not just 
// type parameters.

// Some cases that a where clause is useful:
// 1. When specifying generic types and boudns separately is clearer:
// impl <A: TraitB + TraitC, D: TraitE + TraitF> MyTraiT<A, D> for YourType {}

// Expresing bounds with a where clause:
// impl<A, D> MyTrait<A,D> for YourType where
//   A: TraitB + TraitC,
//   D: TraitE + TraitF {}

// 2. When using a where clause is more expressive than using normal syntax. The impl in this example
// cannot be directly expressed without a where clause:
use std::fmt::Debug;

trait PrintInOption {
  fn print_in_option(self);
}

impl<T> PrintInOption for T where
  Option<T>: Debug {
    // When want Option<T>: Debug as our boudn because that is what's being printed. Otherwise it would
    // be a wrong bound.
    fn print_in_option(self) {
      println!("{:?}", Some(self));
    }
  }

  fn main() {
    let vec = vec![1,2,3];
    vec.print_in_option();
  }
