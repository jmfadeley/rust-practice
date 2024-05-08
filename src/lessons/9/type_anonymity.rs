/// Lessons "9.2.3 Type Anonymity"
/// https://doc.rust-lang.org/rust-by-example/fn/closures/anonymity.html

// Closures capture variables from enclosing scopes. This has consequences. 
// The following requires generics because of how they are defined:

fn apply<F>(f: F) where 
F: FnOnce() {
  f();
}

// When a closure is defined, the compiler implicitly creates a new anonymous 
// structure to store the captured variables inside, using one of these traits:
// Fn:
// FnMut
// FnOnce
// ... for the unknown type. Since this new type is of unknown type, any usage in a
// function will require generics. But <T>, an unbounded type, would still be ambigious
// and is not allowed. Thus bounding by Fn, FnMut or FnOnce (which this implements) is
// sufficient to specify its type.

// F must implement Fn for a closure which takes no inputs and returns nothing
// exactly what is required for print.
fn apply2<F>(f: F)  where
  F: Fn() {
    f();
  }

  fn main() {
    let x = 7;

    // Capture x into an anon type and implement Fn on it. Store it as print.

    let print = || println!("{}", x);

    apply2(print);

    apply(print);
  }