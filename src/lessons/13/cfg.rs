/// Lessons "13.3 cfg"
/// Sources: https://doc.rust-lang.org/rust-by-example/attribute/cfg.html

/* Configuration conditional check sare possible through 2 different operators:
  - the `cfg` attribute: #[cfg(...)]
  - the `cfg!` macro: cfg!(...) in boolean expressions.

  The former is for compilation while the latter is for runtime. They both use the same syntax.

  cfg! does not remove code while #[cfg()] does.
*/

#[cfg(target_os = "linux")]
fn are_you_on_linux() {
  println!("You are running Linux!");
}

#[cfg(not(target_os ="linux"))]
fn are_you_on_linux() {
  println!("You are not on Linux.");
}

fn main() {
  are_you_on_linux();

  println!("Are you sure?");
  if cfg!(target_os = "linux") {
    println!("Yep, Linux.");
  } else {
    println!("Not Linux.");
  }
}