/// Lessons "8.5.1.4 Destructuring pointers/ref"
/// https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring/destructure_pointers.html

// Pointers require a distinction between destructuring and dereferencing as they are different concepts
// used from languages like C/C++. 

// Derefrencing uses *
// Destructuring uses &, ref, and ref mut
fn main() {
    // Assign a reference of type `i32`. The `&` signifies there is a reference being assigned.
    let reference = &4;

    match reference {
        // If `reference` is a pattern matched against `&val`, it results in a comparison like:
        // `&i32`
        // `&val`
        // If the matching 7s are dropped, then the i32 should be assigned to val.
        &val=> println!("Gimme the goods: {:?}", val),
    }

    // To avoid the &, you deference before matching with the * 
    match *reference {
        val => println!("Dereference: {:?}", val),
    }

    // Hmmm...
    match reference {
        val => println!("What happens if... {:?}", val),
    }

    // What if it's not a reference?
    let _not_a_ref = 3;

    // Rust provides `ref` to deal with this, which modifies the assignment.
    let ref _is_a_reference = 3;

    // By defining 2 values without ref, references can be retrived via `ref` and `ref mut`
    let value = 5;
    let mut mut_value = 6;

    // ref.
    match value {
        ref r => println!("Got us a ref do we? {:?}", r),
    }

    // ref mut
    match mut_value {
        ref mut m => {
            // Got a reference, gotta deref it before we can add anything to it.
            *m += 10;
            println!("We added 10 `mut_value`: {:?}", m);
        }
    }

    // But what if we don't? Throws error.
    // let mut nut_value = 7;

    // match nut_value {
    //     ref mut n => {
    //         // Got a reference, gotta deref it before we can add anything to it.
    //         n += 10; // Throws error.
    //         println!("We added 10 `mut_value`: {:?}", n);
    //     }
    // }
    
}