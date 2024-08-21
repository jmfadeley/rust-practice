/// Lessons "14.8 Associated Items, 14.8.1 the Problem "
/// https://doc.rust-lang.org/rust-by-example/generics/assoc_items.html
/// https://doc.rust-lang.org/rust-by-example/generics/assoc_items/the_problem.html

/*
  Associated items refers to a set of rules pertaining ot items of various types. It is an extension to trait generics, and allows
  traits to internally define new items.

  One such item is called an associated type. Providing simpler usage patterns when the trait is generic over its container type.

  A trait that is generic over its container type has type specification requirements - users of the trait MUST specifiy all of its generic
  types. Below, the Contains trait allows the user of the generic types A and B. The trait is then implemented for the Container type, 
  specifying i32 for A and B so that it can be used with fn difference().

  Because Contains is generic, we are focrced to explicitly state all of the generic types for fn difference(). In practice, we want a way to express
  that A and B are determined by the input c. All you will see in the next section, associated types provide exactly that capability.

*/

struct Container(i32, i32);

// a trait which checks if 2 items are stored inside the Container.
// Also retrives first or last value.
trait Contains<A, B> {
  fn contains(&self, _: &A, _: &B) -> bool; // Explicitly requires A and B.
  fn first(&self) -> i32; // Doesn't explicitly require A or B.
  fn last(&self) -> i32; // Doesn't explicitly require A or B.
}

impl Contains<i32, i32> for Container {
  // True if the nubmers stored are equal.
  fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
    (&self.0 == number_1) && (&self.1 == number_2)
  }

  fn first(&self) -> i32 { self.0 }
  fn last(&self) -> i32 { self.1 }
}

fn difference<A, B, C> (container: &C) -> i32 where
  C: Contains<A,B> {
    container.last() - container.first()
}

fn main() {
  let number_1 = 3;
  let number_2 = 10;

  let container = Container(number_1, number_2);

  println!("Does container contain {} and {}: {}", &number_1, &number_2, container.contains(&number_1, &number_2));
  println!("First number: {}", container.first());
  println!("Last number: {}", container.last());

  println!("The difference is: {}", difference(&container));
}