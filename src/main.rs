  // let <var>:<type> = <value>
  // mut = mutable
  // const = constant
  // <something>! = call a macro


fn sample() {
  let x: &str = "A string";
  let y: i32 = 32;
  let mut z: String = String::from("asmr");
  println!("{}", z);
  z = z + "rhs";
  println!("{}", z);
  println!("Hello, world {x}, {}!", y);
}

fn bum_flummox() {
  let x: String = String::from("jenkins?");
  println!("LEROY {x}");
}

// You can compile with "rustc main.rs"
// Then run with "./main"
// Main is kind of a bad name because it causes confusion with a reserved word...
fn main() {
  sample();
  bum_flummox()
}