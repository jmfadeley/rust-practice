/// Lessons "13.1 dead_code"
/// Sources: https://doc.rust-lang.org/rust-by-example/attribute/unused.html

// The dead_code attribute warns of unused functions via the linter. Note that dead_code should be eliminated.
fn used_function() {
  print!("Used.\n");
}

#[allow(dead_code)]
fn unused_function() {}

// #[allow(dead_code)]
fn noisy_unused_function() {}
// FIXME ^ Add an attribute to suppress the warning.

fn main() {
  used_function();
}