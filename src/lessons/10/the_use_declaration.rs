/// Lessons "10.3 The use declaration"
/// https://doc.rust-lang.org/rust-by-example/mod/use.html

// The use declaration can be used to bind a full path to a new name for easier access. This is often how it works:
// use crate::deeply::nested::{
//   my_first_function,
//   my_second_function,
//   AndATraitType
// };

// fn main() {
//   my_first_function();
// }

// The as keyword can bind them as a different name:

use deeply::nested::function as other_function;

mod deeply {
  pub mod nested {
    pub fn function() {
      println!("called `deeply::nested::function()");
    }
  }
}

fn main() {
  // Easier access to said function.
  other_function();

  println!("Entering block");
  {
    // This is equivalent to `use deeply::nested::function as function`.
    // This `function()` will shadow the outer one.

    use crate::deeply::nested::function;

    // use bindings have a local scope. In this case, the shadowing of function is only in this block.
    function();

    println!("Leaving block");
  }
  // function(); // Not found now.
}