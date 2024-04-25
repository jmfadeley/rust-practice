/// Lesson: "3.1 Structures" 
/// Source: https://doc.rust-lang.org/rust-by-example/custom_types/structs.html
/// 
/// Rust custom data types are primarily formed through one of two keywords:
/// struct, which defines a structure
/// enum, an enumeration
/// You can also use const and static keywords

// There are three types of structs:
// Tuple structs, basically named tuples.
// Classis C structs
// Unit structs, field-less and useful for generics.
#[allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit; // AN ABSOLUTE UNIT!

// And the tuple!
struct Pair (i32, f32);

// A struct with 2 fields!
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// You can also use structs to form other structs. Struct-ception.
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

// Not gonna do negatives and abs, it will drive me nuts to look up those libraries right now.
fn rect_area(rect: Rectangle) -> f32 {
    (rect.bottom_right.x - rect.top_left.x) * (rect.top_left.y - rect.bottom_right.y)
}

fn square(point: Point, multiplier: f32) -> Rectangle {
    Rectangle {
        top_left: Point {..point},
        bottom_right: Point {
            x: point.x * multiplier,
            y: point.y * multiplier,
        }
    }
}

fn main() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person {name, age};

    println!("{:?}", peter);

    // Instantiate a point.
    let point: Point = Point { x: 10.3, y: 0.4 };

    // I wonder if these should be accessed through getters and setters instead.
    println!("Point coordinates: ({}, {})", point.x, point.y);

    //Make a new point by using struct update syntax to use the fields of our other one.
    let bottom_right = Point { x:5.2, ..point }; // So we're saying "use point's y value here"

    println!("Second point: ({}, {})", bottom_right.x, bottom_right.y);

    let top_left = Point { ..point }; // Applies to both.
    println!("Third point: ({}, {})", top_left.x, top_left.y);

    // This won't work, it's like the spread operator in JS, and has to be the last arg.
    // let top_whatever = Point { ..point, y:4.4};

    // Works fine though.
    let top_whatever = Point { x:point.x, y:7.2};
    println!("Fourth point: ({}, {})", top_whatever.x, top_whatever.y);

    // You can also destructure the point using a `let` binding. This confuses me.
    let Point { x: left_edge, y: top_edge } = point; // Oh I get it, it's variable assignment.
    println!("LE POINT: ({}, {})", left_edge, top_edge); // So left_edge and top_edge are now vars.

    let _rectangle = Rectangle {
        // Struct instantion is an expression too.
        top_left: Point {x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    println!("Stuff {:?}", _rectangle.top_left);
    println!("Stuff 2 {:?}", _rectangle.bottom_right);

    // Instantiating a unit struct.
    let _unit = Unit;

    // ... And a tuple struct.
    let pair = Pair(1, 0.1);

    println!("Pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;

    println!("Pair contains {:?} and {:?}", integer, decimal);

    let _barf = Rectangle {
        top_left: Point {x: 2.0, y: 8.0},
        bottom_right: Point {x: 5.0, y: 2.0},
    };

    let _vomit = Point {
        x: 5.0,
        y: 4.0,
    };

    println!("Oh boy {:?}", rect_area(_barf));

    let burp = square(_vomit, 3f32);
    println!("Oh square {:?} and {:?}", burp.top_left, burp.bottom_right);
}