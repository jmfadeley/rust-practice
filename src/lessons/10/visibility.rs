/// Lessons "10.1 Visibility"
/// https://doc.rust-lang.org/rust-by-example/fn/diverging.html

// Modules can be used to hierarchically split code in logical units (modules) and manage visibility (public/private) between them.

// It is a collection of items: functions, structs, traits, impl blocks, and even other modules.

// By default items in a module have private visibility. Pub can override that, and only through this can they be accessed outside the module scope.

mod my_mod {
  // private by default.
  fn private_function() {
    println!("called `my_mod::private_function()`");
  }

  // pub overrides this and makes it public
  pub fn function() {
    println!("called `my_mod::function()`");
  }

  // Items can access other items in the same module even when private.
  pub fn indirect_access() {
    println!("called `my_mod::indirect_access()`, that\n");
    private_function();
  }

  pub mod nested {
    pub fn function() {
      println!("called `my_mod::nested::function()`");
    }

    #[allow(dead_code)]
    fn private_function() {
      println!("called `my_mod::nested::private_function()`");
    }
    // Functions declared using `pub(in path)` syntax are only visible within the given path.
    // path must be a parent or ancestor module
    pub(in crate::my_mod) fn public_function_in_my_mod() {
      println!("called `my_mod::nested::public_function_in_my_mod()`");
      public_function_in_nested();
    }

    // Functiond declared during pub(self) syntax are only visible within the current module
    // which is the same as leaving them private
    pub(self) fn public_function_in_nested() {
      println!("called `my_mod::nested::public_function_in_nested()`");
    }

    // Functions declared using pub(super) are only visible in the parent module.
    pub(super) fn public_function_in_super_mod() {
      println!("called `my_mod::nested::public_function_in_super_mod()`");
    }
  }

  pub fn call_public_function_in_my_mod() {
    print!("called `my_modcall_public_function_in_my_mod(), that\n");
    nested::public_function_in_my_mod();
    print!("> ");
    nested::public_function_in_super_mod();
  }

  // pub(crate) makes functions visible only within the current crate
  pub(crate) fn public_function_in_crate() {
    println!("called `my_mod::public_function_in_crate()`");
  }

    // Nested modules follow the same rules for visibility
  mod private_nested {
    #[allow(dead_code)]
    pub fn function() {
        println!("called `my_mod::private_nested::function()`");
    }

    // Private parent items will still restrict the visibility of a child item,
    // even if it is declared as visible within a bigger scope.
    #[allow(dead_code)]
    pub(crate) fn restricted_function() {
        println!("called `my_mod::private_nested::restricted_function()`");
    }
  }
}

fn function(){
  println!("called `function()` are we there yet.");
}

fn main() {
  // Modules allow disambiguation between items that have the same name.
  function();
  my_mod::function();

  // Public items including those inside nested modules, can be accessed from outside the parent module.
  my_mod::indirect_access();
  my_mod::nested::function();
  my_mod::call_public_function_in_my_mod();

  // pub(crate) items can be called from anywhere in the same crate
  my_mod::public_function_in_crate();

    // pub(in path) items can only be called from within the module specified
    // Error! function `public_function_in_my_mod` is private
    //my_mod::nested::public_function_in_my_mod();
    // TODO ^ Try uncommenting this line

    // Private items of a module cannot be directly accessed, even if
    // nested in a public module:

    // Error! `private_function` is private
    //my_mod::private_function();
    // TODO ^ Try uncommenting this line

    // Error! `private_function` is private
    //my_mod::nested::private_function();
    // TODO ^ Try uncommenting this line

    // Error! `private_nested` is a private module
    //my_mod::private_nested::function();
    // TODO ^ Try uncommenting this line

    // Error! `private_nested` is a private module
    //my_mod::private_nested::restricted_function();
    // TODO ^ Try uncommenting this line
}