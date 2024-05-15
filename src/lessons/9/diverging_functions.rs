/// Lessons "9.4 Diverging Functions"
/// https://doc.rust-lang.org/rust-by-example/fn/diverging.html

// Diverging functions don't return. They are marked with ! which is an empty type.

#[warn(dead_code)]
fn foo() -> ! {
  panic!("This call never returns.");
}

// This one cannot be instantiated because the set of all possible values this type can have is empty. Note that it is different from the
// () type (which is the default return I think?) which has exactly one possible value.

// This function returns as usually but there is no info in the return value.
fn some_fn() {
  ()
}

fn main() {
  let _a: () = some_fn();
  println!("This function returns and you can see this line.");
  // foo();
  // ^^ would make the below unreachable.

  // Although this might seem like an abstract concept, it is in fact very useful and often handy. The main advantage
  // of this type is that it can be cast to any other one and therefore used at places where an exact type
  // is required, like instance in match branches. This allows us to write code like this:

  fn sum_odd_numbers(up_to: u32) -> u32{
    let mut acc = 0;
    for i in 0..up_to {
      // Notice that the return type of this match expression must be u32 because of the type of the "addition" variable.
      let addition: u32 = match i%2 == 1 {
        true => i,
        // On the other hand, the "continue" expression does not return u32 but it is still fine because it never returns
        // and therefore does not violete the type requirements of the match expression.
        false => continue,
      };
      acc += addition;
    }
    acc
  }
  println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));
  foo();
  // Because we do nothing else, this is now fine.
}

// As opposed to this function which uses the never_type, and will never return the control back to the caller.
// #![feature(never_type)]
// fn new_sample() {
//   let x: ! = panic!("This call never returns");
//   println!("You will never see this line!")
//   // ^^^ unreachable, compiler fails.
// }