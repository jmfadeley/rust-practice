/// Lessons "14.2 Implementation"
/// https://doc.rust-lang.org/rust-by-example/generics/impl.html

// Similar to functions, implementations require care to remain generic:

#[allow(dead_code)]
struct S; // Concrete type
#[allow(dead_code)]
struct GenericVal<T>(T); // Generic type `GenericVal`

impl GenericVal<f32> {} // Specify f32
impl GenericVal<S> {} // Specify S as above.

// `<T>` must precede the type to remain generic
impl<T> GenericVal<T> {}

struct Val {
  val: f64,
}

struct GenVal<T> {
  gen_val: T,
}

impl Val {
  fn value(&self) -> &f64 {
    &self.val
  }
}

impl<T> GenVal<T> {
  fn value(&self) -> &T {
    &self.gen_val
  }
}

fn main() {
  let x = Val { val: 3.0 };
  let y = GenVal { gen_val: 3i32 };
  println!("{}, {}", x.value(), y.value());
}