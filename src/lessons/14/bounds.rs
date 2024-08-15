/// Lessons "14.4 Bounds"
/// https://doc.rust-lang.org/rust-by-example/generics/bounds.html

// When working with generics, the type parameters often must use traits as "bounds" to stipulate what
// functionality a type implements. For example, the following example uses the trait `Display` to print
// and so it requires T to be bound by Display; T MUST implement Display.

// fn printer<T: Display>(t:T) {
//   println!("{}", t);
// }

// Bounding restrict the generic to types that conform to the bounds like this:
// struct S<T: Display>(T);

// Error! `Vec<T>` does not implement `Display`. This specialization will fail.
// const s = S(vec![1]);

// Another effect of bounding is that generic instances are allowed to access the methods of traits 
// specified in the bounds:

use std::fmt::Debug;
trait HasArea {
  fn area(&self) -> f64;
}

impl HasArea for Rectangle {
  fn area(&self) -> f64 { self.length * self.height }
}

#[derive(Debug)]
struct Rectangle { length: f64, height: f64 }
#[allow(dead_code)]
struct Triangle { length: f64, height: f64 }

// The generic T must implement Debug. Regardless of the type, this will work properly.
fn print_debug<T: Debug>(t: &T) {
  println!("{:?}", t);
}

// T must implement HasArea. Any type which meets the bound can access HasArea's function area.
fn area<T: HasArea>(t: T) -> f64 { t.area() }

fn main() {
  let rectangle = Rectangle { length: 3.0, height: 4.0 };
  let _triangle = Triangle { length: 3.0, height: 4.0 };

  print_debug(&rectangle);
  println!("Area: {}", area(rectangle));
}