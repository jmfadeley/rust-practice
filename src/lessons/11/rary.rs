/// Lessons "11.1 Creating a Library"
/// https://doc.rust-lang.org/rust-by-example/crates/lib.html

// To create a library and see how it links to another crate.
// Use with:
// rustc --crate-type=lib rary.rs
// ls lib*
// library.rlib
pub fn public_function() {
  println!("called rary's `public_function()`");
}

fn private_function() {
  println!("called rary's `private_function()`");
}

pub fn indirect_access() {
  print!("called rary's `indirect_access()`, that\n> ");

  private_function();
}

// Libraries get prefix with "lib" and by default get named after their crate file. But
// This can be overridden by passing the `--crate-name` option. 