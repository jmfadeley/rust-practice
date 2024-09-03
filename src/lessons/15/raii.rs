/// Lessons "15.1 RAII"
/// https://doc.rust-lang.org/rust-by-example/scope.html
 
/* Scopes play an important part in ownership, borrowing and lifetimes. That is, they indicate to the
compiler when borrows are valid, when resources can be freed, and when variables are created or destroyed.

Variables in Rust do more htan just hold data in the stack, they also own resources like Box<T> owns memory in the heap.
Rust enforces RAII (Resource Acquisition is Initialization) so when an object goes out of scope, its destructor is called
and its owned resources are freed. This behavior shields against resource leak bugs, so you'll never have to manually
free memory or worry about memory leaks again. Examples:*/

// raii.rs
struct ToDrop;

// Drop is the function that is called when a memory goes out of scope. By implementing this trait, you 
// can override the behavior.
impl Drop for ToDrop {
  fn drop(&mut self) {
    println!("ToDrop is being dropped.\n");
  }
}

fn create_box() {
  // Allocate an integer on the heap:
  let _box1 = Box::new(3i32);

  // `_box1` is destroyed here and the memory freed.
}

fn main() {
  // Allocate an integer on the heap:
  let _box2 = Box::new(5i32);

  // A nested scope: 
  {
    let _box3 = Box::new(4i32);

    // _box3 is destroyed here and the memory returned.
  }

  // For fun:
  for _ in 0i32 .. 1_000 {
    create_box();
  }

  let x = ToDrop;
  println!("Made a ToDrop!");
}

/* 
Finall you can always double check for memory errors using valgrind, a command line tool:
rustc raii.rs && valgrind ./raii */