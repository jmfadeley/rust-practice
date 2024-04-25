/// Lesson: "3.2 Enums" 
/// Source: https://doc.rust-lang.org/rust-by-example/custom_types/enum.html

// enum allows for creation of a type that is of only a few different variants.
enum WebEvent {
    // Unit like:
    PageLoad,
    PageUnload,
    // tuple struct type
    KeyPress(char),
    Paste(String),
    // Or C-like:
    Click { x: i64, y: i64 },
}

// A function that taks a WebEvent enum as an arg and returns nothing.
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("press '{}'", c), // Desctructured c
        WebEvent::Paste(s) => println!("pasted \"{}\"", s),
        // They use brackets, but let's see if I can skip that.
        WebEvent::Click {x, y} => println!("clicked at x={}, y={}.", x, y),
    }
}

// You can also use alias for verbose names.
enum TheLongestEnumYoullSeeTodayVeryBadDoNotDoThis {
    Add,
    Subtract,
}

impl TheLongestEnumYoullSeeTodayVeryBadDoNotDoThis {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

type Operations = TheLongestEnumYoullSeeTodayVeryBadDoNotDoThis;

fn main() {
    let pressed = WebEvent::KeyPress('x');
    // to_owned creates an owned String from a string slice.
    // Basically it clones it but is not object safe.
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;
    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    let x = Operations::Add;
    let y = Operations::Subtract;

    println!("{}", x.run(3, 4));
    println!("{}", y.run(5, 7));
}