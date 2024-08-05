/// Lessons "14.1 Functions"
/// Sources: https://doc.rust-lang.org/rust-by-example/generics/gen_fn.html

/*
The same rules apply to funcitons when they are preceded with <T>. This is useful when the funciton returns a generic type or if there's not
enough info to infer the types.
 */

struct A;
struct S(A);
struct SGen<T>(T);

// The following functions all take ownership of the variable passed into them and immediately go out of scope, freeing the variable.

// Define a function `reg_fn` that takes argument _s of type S. 
// this is not generic.
fn reg_fn(_s: S) {}

// `gen_spec_t` takes argument `_s` of type `SGen<T>`. It has been explicitly given the type paramter A, but because A has not been 
// specified as a generic type parameter for `gen_spec_t` it is not generic.
fn gen_spec_t(_s: SGen<A>) {}

// Define a function `gen_spec_i32` that takes an argument `_s` of type `SGen<i32>`.
// It has been explicitly given the type parameter `i32`, which is a specific type.
// Because `i32` is not a generic type, this function is also not generic.
fn gen_spec_i32(_s: SGen<i32>) {}

// Define a function `generic` that takes an argument `_s` of type `SGen<T>`. Because `SGen<T>` is preceded by `<T>`, this fn is 
// generic over `T`
fn generic<T>(_s: SGen<T>) {}

fn main() {
  reg_fn(S(A)); // concrete.
  gen_spec_t(SGen(A)); // Implicitly specified type parameter A.
  gen_spec_i32(SGen(6)); // Implicitly specific type parameter i32.

  // Explicitly specified type parameter `char` to `generic()`.
  generic::<char>(SGen('a'));

  // Implicitly specified type paramter `char` to `generic()`.
  generic(SGen('c'));
}