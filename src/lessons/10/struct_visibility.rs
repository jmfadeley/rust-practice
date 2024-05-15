/// Lessons "10.2 Struct visibility"
/// https://doc.rust-lang.org/rust-by-example/mod/struct_visibility.html

// Structs have an exta level of visibility with their fields. Visibility defaults to private and can be overridden
// with the pub modifier. Hiding info is encapsulation

mod my {
  pub struct OpenBox<T> {
    pub contents: T,
  }

  // Field with priate field of generic type T
  pub struct ClosedBox<T> {
    contents: T,
  }

  // Pub constructor method
  impl<T> ClosedBox<T> {
    pub fn new(contents: T) -> ClosedBox<T> {
      ClosedBox {
        contents: contents,
      }
    }
  }
}

fn main() {
  // pub structs with pub fields can be constructed as usual
  let open_box = my::OpenBox { contents: "public info" };

  // and their fields normally access.
  println!("This contains: {}", open_box.contents);

  // This won't work cause it's private
  // let closed_box = my::ClosedBox{ contents: "classified info" };

  // Pub constructor is key.
  let _closed_box = my::ClosedBox::new("classified info");

  // Private fields cannot be accessed either. Use pub functions within
  // println!("Closed box info: {}, _closed_box.contents")

}