/// Lessons "15.2.2 Partial Moves"
/// https://doc.rust-lang.org/rust-by-example/scope/move/partial_move.html
 
/* Within the destructing of a single variable, both by-move and by-reference pattern bindings can 
be used at the same time. Dping this will result in a partial move of the variable, which means that 
parts of the variable will be moved while other parts stay. In such a case, the parent variable cannot
be used afterwards as a whole, however the parts that are only references (and not moved) can still be
used. */

fn main() {
  #[derive(Debug)]
  struct Person {
    name: String,
    age: Box<u8>,
  }

  let person = Person {
    name: String::from("Alice"),
    age: Box::new(20),
  };

  println!("The person struct is {:?}", person);

  // `name` is moved out of person, but `age` is referenced.
  let Person { name, ref age } = person;

  println!("The person's age is {}", age);
  println!("The person's name is {}", name);

  // Error! Borrow of partially moved value: `person` partial move occurs
  // println!("The person struct is {:?}", person);

  println!("The person's age from person struct is {}", person.age);
}