/// Lessons "8.4 For and Range"
/// Sources: https://doc.rust-lang.org/rust-by-example/flow_control/for.html
 
// `for in` construct runs through an iterator. You can just do `a..b` range notation
// where a is inclusive and b exclusive.

// a..=b is inclusive on both ends.

fn main() {
    for n in 1..101 { // U=p to but not including 101, so 100 tops.
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        } // Last line is buzz.
    }

    for n in 1..=101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        } // Last line is 101.
    }

    // `for..in` can interact with Iterator. Your standard `for` loop
    // uses `into_inter` under the hood. But it's not hte only way.
    // `into_iter`, `iter` and `iter_mut` all handle conversion differently.

    let names = vec!["Bob", "Frank", "Buehler"];
    for name in names.iter() { // Borrows the value but does not modify. 
        match name {
            &"Buehler" => println!("Buehler? Buehler? Buehler?"),
            _ => println!("{} accounted for.", name),
        }
    }
    println!("names: {:?}", names);

    // `into_iter` consumes the collection and thus it cannot be reused.
    // Remember this is the default!
    for name in names.into_iter() {
        match name {
            "Buehler" => println!("There you are, Buehler"),
            _ => println!("No one cares, {}.", name),
        }
    }

    let mut new_names = vec!["Sara", "Anderson", "Buehler"];

    for name in new_names.iter_mut() {
        *name = match name {
            &mut "Buehler" => "He took a day off.",
            _ => "Get back to class.",
        }
    }
    println!("names: {:?}", new_names);
    
    // println!("names: {:?}", names);
    // The above will not work as it has been used by the loop.
}