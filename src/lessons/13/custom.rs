/// Lessons "13.3.1 config"
/// https://doc.rust-lang.org/rust-by-example/attribute/cfg/custom.html

/*
  Some conditions like target_os are implicitly provided by rustc but custom conditionals must be passed
  to rustc using the --cfg flag.
*/

#[cfg(some_condition)]
fn condiitonal_function() {
  println!("Conditions met!");
}

fn main() {
  condiitonal_function();
}

// Use `rustc --cfg some_condition custom.rs && ./custom`