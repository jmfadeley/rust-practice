mod inaccessible;
pub mod nested;

// `mod inaccessible` and `mod nested` will located the `nested.rs` and `inaccessible.rs` files and insert them here under their respective
// modules;
pub fn function() {
  println!("called `my::function()`");
}

fn private_function() {
  println!("called `my::private_function`");
}

pub fn indirect_access() {
  print!("called `my::indirect_access()`, that\n> ");
  private_function();
}