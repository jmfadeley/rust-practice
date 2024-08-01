/// Lessons "13.2 Crates"
/// Sources: https://doc.rust-lang.org/rust-by-example/attribute/crate.html

/* The crate_type attribute can be used to tell the compiler whether a crate is a binary or a library
(and even which type of library) and the crate_name attribute can be used to set the name of the crate.

Note that both the crate_type and crate_name attributes have NO effect whatsoever when using Cargo, the Rust
package manager. Since Cargo is used for teh majority of Rust projects, this means real world uses of crate_type 
and crate_name are pretty limited.
*/ 

// NOTE: The sample code does NOT work as is. 
// This crate is a library
#[crate_type = "lib"]
// The library is named "rary"
#[crate_name = "rary"]
pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}
  
fn main() {}
