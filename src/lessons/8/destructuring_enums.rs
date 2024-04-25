/// Lessons "8.5.1.2 Destructuring Enums"
/// https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring/destructure_enum.html

// Matching enums oh boy!
// `allow` required to silence warnings because only one variant is used.
#[allow(dead_code)]
enum Color {
    Red, // These 3 are specified solely by their name.
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn main() {
    let color = Color::CMYK(122, 17, 40, 9);

    println!("Gimma a col'ah!");
    match color {
        Color::Red => println!("The color is red!"),
        Color::Blue => println!("The color is blue!"),
        Color::Green => println!("The color is green!"),
        Color::RGB(r, g, b) => println!("red: {}, blue: {}, green: {}!", r, g, b),
        Color::HSV(h, s, v) => println!("hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) => println!("hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) => println!("cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) => println!("cyan: {}, magenta: {}, yellow: {}, key (black): {}!", c, m, y, k),
        // All paths covered, so no need for another branch.
    }
}