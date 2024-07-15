/// Lessons "11.2 Using a Library"
/// https://doc.rust-lang.org/rust-by-example/crates/using_lib.html

// To link a crate to this new library, you need to use the `--extern` flab.

// rustc using_lib.rs --extern rary=library.rlib && ./using_lib

fn main() {
  rary::public_function();

  // Error! `private_function` is private.
  // rary::private_function();

  rary::indirect_access();
}