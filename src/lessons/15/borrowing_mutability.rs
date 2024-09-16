/// Lessons "15.3.1 Mutability (Borrowing)"
/// https://doc.rust-lang.org/rust-by-example/scope/borrow/mut.html
 
/* Mutable data can be mutably borrowed using &mut T. T his is called mutable reference and gives
read/write access to the borrower. In contrast, &T borrows the data via an immutable reference, and 
the borrower can read the data but not modify it. */
 #[allow(dead_code)]
 #[derive(Clone, Copy)]
 struct Book {
  // `&'static str` is a refernece to a string in read only memory.
  author: &'static str,
  title: &'static str,
  year: u32,
 }

 // this function takes a reference to a book:
 fn borrow_book(book: &Book) {
  println!("I immutably borrowed {} - {} edition.", book.title, book.year);
 }

 fn new_edition(book: &mut Book) {
  book.year = 2014;
  println!("I mutably borrowed {} - {} edition.", book.title, book.year);
 }

 fn main() {
  // Create an immutable book named `immutabook`
  let immutabook = Book {
    author: "William H. Keith Jr",
    title: "Battletech: Decision at Thunder Rift",
    year: 1986,
  };

  // Create a mutable copy.
  let mut mutabook = immutabook;

  // Immutably borrow an immutable object.
  borrow_book(&immutabook);

  // Immutably borrow an mutable object.
  borrow_book(&mutabook);

  // Borrow a mutable object as mutable.
  new_edition(&mut mutabook);

  // Now we error. Guess why.
  // new_edition(&mut immutabook);
  // ^ Comment me out.
 }