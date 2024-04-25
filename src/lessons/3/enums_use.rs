/// Lesson: "3.2.1 Enums use" 
/// Source: https://doc.rust-lang.org/rust-by-example/custom_types/enum/enum_use.html

#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
    Modest,
}

enum Work {
    Civilian, 
    Soldier,
}

fn main() {
    // Explicitly `use` each name so they are available without manual scoping.
    use crate::Status::{Poor, Rich};
    // Automatically `use`` each name instead `Work`.
    use crate::Work::*;

    // Same as Status::Poor, less verbose.
    let status = Poor;
    // Same as Work::Civilian.
    let work = Civilian;

    // Errors. Was not made available.
    // let secondStatus = Modest;

    match status { // No scoping needed thanks to use.
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
        Status::Modest => println!("They won't be buying the nicest shoes."),
    }

    match work { // Lack of scoping again.
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldier fight!"),
    }
}