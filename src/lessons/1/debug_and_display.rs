/// Lesson: "1.2 Formatted Print" and all 3 sections. 
/// Source: https://doc.rust-lang.org/stable/rust-by-example/hello/print.html

use std::fmt;

// Printing is handled by a series of macros defined in std:fmt.
fn main() {
    debug_lesson();
    println!();
    display_lesson();
    display_advanced_lesson();
    formatting_lists();
}

// Types that want to use std::fmt formatting traits require an implementation to be printable.
// Automatic implementation is only provided for types such as in std library. All others must
// be manually implemented somehow.

// fmt::Debug trait (kind of like annotation?) makes this easy as all types can derive 
// (automatically create) the fmt::Debug implementation. fmt::Display cannot and must be manually
// implemented.
fn debug_lesson() {
    println!("debug_lesson time!");
    // Can't be printed with with either fmt:Display or fmt::Debug.
    // struct Unprintable(i32); // Will throw a warning!
    
    // The derive attribute automatically creates the implementation required to make this struct
    // printable with fmt::Debug
    #[derive(Debug)] 
    struct DebugPrintable(i32); 
    
    // {:?} is similar to {}. All std library types are automatically printable with {:?} too.
    println!("{:?} months to a year", 12);
    println!("{1:?} {0:?} is the {guy:?}!", "Suavez", "Rico", guy="dude");

    // Compare this to normal:
    println!("{1} {0} is the {guy}!", "Suavez", "Rico", guy="dude");

    // This will throw an error due to lack of fmt:Display or fmt:Debug.
    // println!("Now {:?} will print! WOO!", Unprintable(3));

    // This will work!
    println!("Now {:?} will print! WOO!", DebugPrintable(3));

    // Won't work! {} uses default formatter, which this cannot make sense of.
    // println!("Now {} will print! WOO!", DebugPrintable(3));

    // One can try {:#?} for pretty print.
    println!("Now {:#?} will print! WOO!", DebugPrintable(3));

    // I know what you're thinking. Why not derive(Display)? It won't work because it doesn't
    // implement std::fmt::Display
    // #[derive(Display)]
    // struct DisplayPrintable(i32);
    // println!("BEHOLD {}", DisplayPrintable(5));

    // However, we can manually implemented it... see display_lesson.
}

struct Structure(i32);

// fmt::Debug is ugly. It's usually better to customize the output using fmt::Display
// Normally this crap wouldn't be below the main program, but for learning purposes...
impl fmt::Display for Structure { // Note that this is for THIS PARTICULAR variable.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Writes strictly the first output of supplied stream 'f'. 
        // Returns fmt::Result, which states whether the op failed or not.
        write!(f, "{}", self.0) // Same syntax as println! No semicolon so we return this.
    }
}

fn display_lesson() { // Note that we don't have to use the derive attribute.
    println!("BEHOLD {}", Structure(5)); // Voila.
    // But we may want to write more interesting fmt::Displays for various generics.
}

#[derive(Debug)] // To contrast to Display.
struct MinMax(i64, i64);


impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}) SHAZAM BABY!", self.0, self.1) // No semicolon so we return this.
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64, // Look Ma! Comma at the end.
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y:{}", self.x, self.y)
    }
}

fn display_advanced_lesson() {
    let minmax = MinMax(0, 14);

    println!("Behold the comparisons:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);
    
    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("Show me the {big} range and the {small} range!", big=big_range, small=small_range);

    let point = Point2D { x: 3.5, y: 4.1 };
    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
    
    // Error. Both `Debug` and `Display` were implemented, but `{:b}`
    // requires `fmt::Binary` to be implemented. This will not work.
    // Try implementing binary when you're not lazy.
    // println!("What does Point2D look like in binary: {:b}?", point);
}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?; // The question mark is for handling all fmt::Results, as in there is more coming.

        // Iterate over v in vec while enumerating the iteration count in count.
        for (count, v) in vec.iter().enumerate() { // count is probably index, while v is value.
            // println!("JAMES{}", count); // Println will also write to the results. I did this to understand.
            // println!("JAMESv{}", v);
            if count !=0 { write!(f, ", ")?; } // More results coming!
            write!(f,"{}", v)?;
        }
        write!(f, "]") // Closing bracket. Note the absence of ? meaning it's concluded. No ; to return
    }
}

// Okay, now for a bit of fun.
fn formatting_lists() {
    let v = List(vec![55, 6, 8, 72]);
    println!("{}", v);
}