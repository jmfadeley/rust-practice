/// Lessons "14 Generics"
/// Sources: https://doc.rust-lang.org/rust-by-example/generics.html

// Generics is about generalizing types and functionalities to broader cases. They're great for reducing code duplication. 
// Generic parameters are usually reprensented as `<T>`.


// A concrete type 'A'
struct A;

// While defining type Single, the fuse use of A is not defined as `<A>`.
// Thus single is a concrete type, and A is defined above.
struct Single(A);

// Here <T> precedes the use of T. So `SingleGen` is generic type. T can be anything,
// including A.
struct SingleGen<T>(T);

fn main() {
  // single is concrete and explicitly takes A.
  let _s = Single(A);

  // The type is explicitly specified but still can be anything.
  let _char: SingleGen<char> = SingleGen('a');

  // SingleGen can also be implicitly specified:
  let _t = SingleGen(A);
  let _i32 = SingleGen(6); // i32 default.
  let _char = SingleGen('a'); // Uses char.
}