/// Lesson: "3.2.3 Testcase: linked-list" 
/// Source: https://doc.rust-lang.org/rust-by-example/custom_types/enum/testcase_linked_list.html

use crate::List::*;

enum List {
    // Cons: Tuple struct that wraps an element and a point to the next node. 
    Cons(u32, Box<List>),
    Nil, //Node that signifies the end of the linked list. Last stop.
}

impl List {
    fn new() -> List {
        Nil // `Nil` has type `List`
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self)) // `Cons` is also a type list.
    }

    fn len(&self) -> u32 {
        // `self` has to be matched, because behavior depends
        // on which version of self. `self` has type `&List` and 
        // `*self` has type `List` matching on concrete type `T`, 
        // which is preferable to `&T` There was an update in 2018
        // such that Rust can use self and no ref on tail.
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0 // If nothing else...
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify()) // Format returns.
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn main() {
    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has a length of: {}", list.len());
    println!("{}", list.stringify());
}