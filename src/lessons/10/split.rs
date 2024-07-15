/// Lessons "10.2 File Hierarchy"
/// https://doc.rust-lang.org/rust-by-example/mod/split.html

// Modules can be mapped to a file/directory hierarchy.
// This declaration will look for a file named my.rs and will insert its contents inside a module named `my` 
// under this scope.

mod my;

fn function() {
  println!("called `function()`");
}

fn main() {
  my::function();

  function();
  
  my::indirect_access();
  my::nested::function();
}