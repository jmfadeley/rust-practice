/// Lessons "9.2.5 As output parameters"
/// https://doc.rust-lang.org/rust-by-example/fn/closures/output_parameters.html

// Closures as outputs are also possible. But you must use the impl trait to return them because
// anonymous closure types are unknown.

// Valid traits for returning a closure are Fn, FnMut, FnOnce

// Aside from this the move keyword must be used which signals all captures occur by value. This is
// required because any captures by reference would be dropped as soon as the function exited leaving
// invalid refs in the closure.

fn create_fn() -> impl Fn() {
  let text= "Fn".to_owned();

  move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut() {
  let text = "FnMut".to_owned();

  move || println!("This is a: {}", text)
}

fn create_fnonce() -> impl FnOnce() {
  let text = "FnOnce".to_owned();
  move || println!("This is a: {}", text)
}

fn main() {
  let fn_plain = create_fn();
  let mut fn_mut = create_fnmut();
  let fn_once = create_fnonce();

  fn_plain();
  fn_mut();
  fn_once();
}