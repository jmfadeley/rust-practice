/// Lesson: "1. Hello World" 
/// Source: https://doc.rust-lang.org/rust-by-example/hello

fn main() {
    // Basics.
    let x: String = format!("Behold Garth! --- "); // Write formatted text to String. 
    print!("{}", x); // Outputs results to io::stdout. Note that as its a variable, it needs {} to format against.
    let y: String = format!("No way, {}!", "Wayne"); // Note the injection this time.
    println!("{}", y); // Same as `print!` but ends with a new line this time. 
    eprint!("Doh!"); // To io:stderr
    eprintln!("Double doh!"); // Same as above, just with new line.
    println!(""); // New line for next section.

    // Arguments.
    println!("{} bottles of root beer.", 40); // Use of integers instead of a string.
    println!("{0} is on {1}. Who? {0}. Then who is on {1}?", "Who", "first"); // Positional arguments.
    println!("{question} {subject}? {answer}", question="Got anymore of that", subject="tutorial", answer="Suitainly!");
    println!(""); 

    // Formatting options
    println!("Base 10:               {}", 69420); // 69420.
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100.
    println!("Base 10 (octal):       {:o}", 69420); // 207454.
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c.
    println!("Base 16 (hexadecimal): {:X}", 69420); // 10F2C.
    println!(""); 

    // Padding.
    println!("{number:>5}", number = 1); // Outputs the text with 4 preceding spaces, then 1.
    println!("{number:>5}", number = 75); // Outputs the text with 3 preceding spaces, then 75.
    println!("{number:<5}{digits}", number = 75, digits=42); // 3 white spaces after 75, then 42.
    println!("{number:0>5}", number=1); // 00001, pads extra zeros.
    println!("{number:0<5}", number=1); // 00001, pads extra zeros afterwards, can misrepresent results.
    println!(""); 

    // Warnings.
    // println!("My name is {0}, {1} {0}", "Bond"); // Compilation error for missing arguments.
    #[allow(dead_code)]
    struct Structure(i32);
    // println!("This struct `{}` won't print...", Structure(3)); 
    // ^^^ Won't compile because Structure doesn't implement fmt::Display

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}"); 
    // Rust 1.58 lets you inject variables from the rest of the code.
    // This displays 4 white spaces and 1. "    1" 
    println!();
    debug_lesson();
}
