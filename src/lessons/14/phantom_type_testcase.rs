/// Lessons "14.9.1 Testcase: unit clarification"
/// https://doc.rust-lang.org/rust-by-example/generics/phantom/testcase_units.html

// A useful method of unit conversion can be examined by implementing add with a phantom type.
// The add trait is examined:

// This construction would impose: self + RHS = output
// where RHS defaults to self if not specified in the implementation.
// pub trait Add<RHS = Self> {
//   type Output;

//   fn add(self, rhs: RHS) -> Self::Output;
// }
// Output must be T<U> so that T<U> + T<U> = T<U>
// impl<U> Add for T<U> {
//   type Output = T<U>;
// }

// Whole thing:

use std::ops::Add;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy)]
enum Inch {}
#[derive(Debug, Clone, Copy)]
enum Mm {}

#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64, PhantomData<Unit>);

// The Add trait defines the behavior of the + operator.
impl<Unit> Add for Length<Unit> {
  type Output = Length<Unit>;

  fn add(self, rhs: Length<Unit>) -> Length<Unit> {
    // + calls the add implementation for f64
    Length(self.0 + rhs.0, PhantomData)
  }
}

fn main() {
  let one_foot: Length<Inch> = Length(12.0, PhantomData);
  let one_meter: Length<Mm> = Length(1000.0, PhantomData);

  // + calls the add method.
  // Because add uses copy, add() does  not conumse one_foot and one_meter but instead
  // copies them into self and rhs.
  let two_feet = one_foot + one_foot;
  let two_meter = one_meter + one_meter;

  println!("one foot + one_foot = {:?} in", two_feet.0);
  println!("one meter + one_meter = {:?} mm", two_meter.0);

  // Compile error mismatch
  // let one_feter = one_foot + one_meter;
}