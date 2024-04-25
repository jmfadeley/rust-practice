/// Lessons "9.1 Methods"
/// https://doc.rust-lang.org/rust-by-example/fn/methods.html

// Some functions are connected to a particular type. 2 forms:
// associated functions and methods. Associated functions are defined
// on a type generally (do they belong to the class?), while methods 
// are for the particular instance of a type.

struct Point {
  x: f64,
  y: f64,
}

impl Point {
  // Associate function because it is associated with Point.
  fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
  }

  // Another associated function.
  fn new(x: f64, y: f64) -> Point {
    Point {x: x, y: y}
  }
}

struct Rectangle {
  p1: Point,
  p2: Point,
}

impl Rectangle {
  // this is a method, &self is sugar for self: &Self, where Self
  // is the type of the caller object (Rectangle in this case).
  fn area(&self) -> f64 {
    let Point { x: x1, y: y1 } = self.p1;
    let Point { x: x2, y: y2 } = self.p2; 

    ((x1 - x2) * (y1 - y2)).abs() 
  }

  fn perimeter(&self) -> f64 {
    let Point { x: x1, y: y1 } = self.p1;
    let Point { x: x2, y: y2 } = self.p2;

    2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
  }

  // this method takes the caller's object to be mutable
  // Thus &mut self desugars mmmm to self: &mut Self
  fn translate(&mut self, x: f64, y: f64) {
    self.p1.x += x;
    self.p2.x += x;

    self.p1.y += y;
    self.p2.y += y;
  }
}

struct Pair(Box<i32>, Box<i32>);

impl Pair {
  // this method "consumes" the resources of the caller object
  // self desguars to self: Self
  fn destroy(self) {
    let Pair(first, second) = self;

    println!("Destroying the Pair({}, {})", first, second);

    //First and second go out of scope and are freed.
  }
}

fn main() {
  // Associated functiosn are called using double colons
  let rectangle = Rectangle {
    p1: Point::origin(),
    p2: Point::new(3.0, 4.0),
  };

  println!("Rectangle perimeter: {}", rectangle.perimeter());
  println!("Rectangle area: {}", rectangle.area());

  let mut square = Rectangle {
    p1: Point::origin(),
    p2: Point::new(1.0, 1.0),
  };

  // Rectangle is immutable, but this method requires a mutable object.
  //rectangle.translate(1.0, 0.0);
  // Uncomment for a bad time.

  square.translate(1.0, 1.0);
  let pair = Pair(Box::new(1), Box::new(2));

  pair.destroy();

  // This causes an error because the prior destroy call consumed it.
  //pair.destroy();
  // Uncomment for a bad time.
}