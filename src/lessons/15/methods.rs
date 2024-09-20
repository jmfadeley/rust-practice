/// Lessons "15.4.3 Methods"
/// https://doc.rust-lang.org/rust-by-example/scope/lifetime/methods.html

/* Methods are annotated similarly like functions. */

struct Owner(i32);

impl Owner {
  // Annotated lifetimes as in a standalone function.
  fn add_one<'a>(&'a mut self) { self.0 +=1; }
  fn print<'a>(&'a self) {
    println!("`print`: {}", self.0);
  }
}

fn main() {
  let mut owner = Owner(18);
  owner.add_one();
  owner.print();
}