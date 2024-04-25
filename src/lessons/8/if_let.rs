/// Lessons "8.6 If Let"
/// Sources: https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html

// For something in a minute.
enum Foo {
  Bar, 
  Baz,
  Qux(u32)
}
// Fixing this.
enum Boo{Far}

fn main() {

  let optional = Some(7);
  // Sometimes matching enums is awkward.
  match optional {
    Some(i) => {
      println!("This is a really long string and `{:?}`", i);
      // ^ Needed 2 indentations just so we could destructure `i` from 
      // the option.
    },
    _ => {},
    // ^ Because match is exhaustive.
  };

  let number = Some(7);
  let letter: Option<i32> = None;
  let emoticon: Option<i32> = None;

  if let Some(i) = number {
    println!("Matched {:?}!", i);
  }

  if let Some(i) = letter {
    println!("Matched {:?}!", i);
  } else { //
    println!("Didn't match a number. Let's go with a letter.");
  }

  // Or we could provided altered failing conditions.
  let i_like_letters = false;

  if let Some(i) = emoticon {
    println!("Matched {:?}, i",i);
  } else if i_like_letters {
    println!("Didn't match a number. Let's go with a letter?");
  } else {
    println!("NO!");
  }

  // You can also match non-parameterized enum variants. This is true even in cases
  // where the enum doesn't implement or derive PartialEq. In such cases if Foo:Bar ==
  // would fail to compile, because instances of enums cannot be equated, however
  // if let would continue to work.

  let a = Foo::Bar;
  let b = Foo::Baz;
  let c = Foo::Qux(100);

  if let Foo::Bar = a {
    println!("a is a foobar");
  }

  // b doesn't match Foo::Bar so this will do nothing.
  if let Foo::Bar = b {
    println!("Guess what's not happening.");
  }

  // Variable cmathces Foo::Qux which has a value similar
  // Some() in the preivous example
  if let Foo::Qux(value) = c {
    println!("c is {}", value);
  }

  // Binding also works with if let.
  if let Foo::Qux(value @ 100) = c { // If c were something other than 100 this won't fire
    println!("c is one hundred: {}", value);
  }

  let d = Boo::Far;

  if let Boo::Far = d {
    println!("d is boo far.");
  }
}