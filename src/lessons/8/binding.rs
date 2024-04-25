/// Lessons "8.5.3 Binding"
/// https://doc.rust-lang.org/rust-by-example/flow_control/match/binding.html

// Indirectly accessing a variable makes it impossible to branch and use that variable without 
// rebinding. Using `@` sigil will allow for binding values to names.

fn age() -> u32 {
  15
}

// You can also destructure enums.
fn some_number() -> Option<u32> {
  Some(42)
}

fn main() {
  println!("Tell us how old you are.");

  match age() { // Note that this is a function, not a variable.
    0 => println!("I was just born."),
    n @ 1 ..= 12 => println!("I am a child of age {:?}", n),
    n @ 13 ..=19 => println!("I am a teen of age {:?}", n),
    n => println!("I'm old at {:?}", n),
  }

  match some_number() {
    Some(n @ 42) => println!("The answer: {}!", n),
    Some(n) => println!("Not interesting... {}", n),
    _ => (),
  }
}