/// Lessons "14.8.2 Associated Types"
/// https://doc.rust-lang.org/rust-by-example/generics/assoc_items/types.html

/*
The use of "Associated types" improves the overall readability of code by moving inner types locally into a trait as output types.
Syntax for the trait definition is as follows: */


// A and B are defined in the trait via the type keyword. Note type in this context is different from type when used for aliases:
// trait Contains {
//   type A;
//   type B;

//   // Updated syntax to refer to these new types generically:
//   fn contains(&self, _:&Self::A, _: &Self::B) -> bool;
// }

// Note that the functions that use the trait Contains are no longer required to express A or B at all:
// fn difference<A, B, C>(container: &C) -> i32 where
//   C: Containers<A, B> { ... }

// Using associated types:
//   fn difference<C: Contains>(container: &C) -> i32 { ... }

// Rewritten from last section:


struct Container(i32, i32);

trait Contains {
  type A;
  type B;

  fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
  fn first(&self) -> i32;
  fn last(&self) -> i32;
}

impl Contains for Container {
  // Specify what A and B are. If the input type is Container(i32, i32) the outputs would be as well.
  type A = i32;
  type B = i32;

  // &Self::A and B are also valid here.
  fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
    (&self.0 == number_1) && (&self.1 == number_2)
  }

  fn first(&self) -> i32 { self.0 }
  fn last(&self) -> i32 { self.1 }
}

fn difference<C: Contains>(container: &C) -> i32 {
  container.last() - container.first()
}

fn main() {
  let number_1 = 3;
  let number_2 = 10;

  let container = Container(number_1, number_2);

  println!("Does container contain {} anmd {}: {}", &number_1, &number_2, container.contains(&number_1, &number_2));

  println!("First number: {}", container.first());
  println!("Last number: {}", container.last());

  println!("The difference is: {}", difference(&container));
}