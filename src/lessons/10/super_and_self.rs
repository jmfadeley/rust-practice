/// Lessons "10.2 super and self"
/// https://doc.rust-lang.org/rust-by-example/mod/super.html

// The super and self ekywords can be used to remove ambiguity when accessing items and to prevent unnecessary hardcoding of paths.

fn function() {
  println!("called `function()`");
}

mod cool {
  pub fn function() {
    println!("called `cool::function()`");
  }
}

mod my {
  fn function() {
    println!("called `my::function()`");
  }

  mod cool {
    pub fn function() {
      println!("called `my::cool::function()`");
    }
  }

  pub fn indirect_call() {
    print!("called `my::indirect_call(), that\n> ");

    // The self keywork refers to teh current module scope. So self::function.
    self::function();
    function();

    // We can also use self to access another module inside of `my`
    self::cool::function();

    // super lets us call the parent scole function.
    super::function();

    // This will bind to the `cool::function` in the *crate* scope.
    // In this case the crate scope is the outermost scope.
    {
      use crate::cool::function as root_function;
      root_function();
    }
  }
}

fn main() {
  my::indirect_call();
}