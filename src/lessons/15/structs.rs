/// Lessons "15.4.4 Structs"
/// https://doc.rust-lang.org/rust-by-example/scope/lifetime/struct.html

/* Annotation of lifetimes in structs are also similar to functions: */

#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

// similarly both refs here must outlive this struct
#[derive(Debug)]
struct NamedBorrowed<'a> {
  x: &'a i32,
  y: &'a i32,
}

// An enum which is either an i32 or a ref to one
#[derive(Debug)]
enum Either<'a> {
  Num(i32),
  Ref(&'a i32),
}

fn main() {
  let x = 18;
  let y = 15;

  let single = Borrowed(&x);
  let double = NamedBorrowed { x: &x, y: &y};
  let reference = Either::Ref(&x);
  let number = Either::Num(y);

  println!("x is borrowed in {:?}", single);
  println!("x and y are borrowed in {:?}", double);
  println!("x is borrowed in {:?}", reference);
  println!("y is NOT borrowed in {:?}", number);
}