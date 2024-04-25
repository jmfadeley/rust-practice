/// Lessons "5.1 Casting"
/// Sources: https://doc.rust-lang.org/rust-by-example/types/cast.html

// Rust provides no implicit type conversion (coercion) between primitive types.
// But you can be explicity in type conversion (casting) by using the `as` keyword.
 #[allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32; // Would be f64 default.
    
    // let integer: u8 = decimal;
    // Uncomment to have a !good time!

    // Let's get explicit, baby.
    let integer = decimal as u8;
    let character = integer as char;

    // Error! There are limits on conversion rules.
    // Why you thought you could convert a float to a char is anyone's guess.
    // let char = decimal as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // When casting any value to an unsigned type, T,
    // T::MAX + 1 is added or subtracted until the value fits into
    // the new type. :O
    println!("1000 as u16 is: {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232.
    // Under the hood, the first 8 least significant bits (LSB) are kept,
    // while the rest towards the most significant bits (MSB) get truncated.
    println!("1000 as u8 is: {}", 1000 as u8);
    // Meanwhile going negative to unsigned  flips around.
    // -1 + 256 = 255.
    println!(" -1 as u8 is: {}", (-1i8) as u8);

    // For positive numbers, this is the same as the modulus.
    println!("1000 mod 256 is: {}", 1000 % 256);

    // when casting to a signed type, the (bitwise) result is the same as 
    // the first casting casting to the coressponding unsigned type. If the most
    // significat bit of that value is 1, then the value is negative.

    // Unless of course it already fits.
    println!("128 as i16 is: {}", 128 as i16);

    // In boundary case 128 value in 8-bit two's complement representation is -128.
    println!("128 as a i8 is: {}", 128 as i8);

    // More:
    println!("127 as a i8 is: {}", 127 as i8); // 127
    println!("129 as a i8 is: {}", 129 as i8); // -127

    // With prior examples:
    // 1000 as u8 = 232
    println!("1000 as a u8 is: {}", 1000 as u8);
    // And the value of 232 in 8-bit two's complement representation is -24.
    println!("232 as a i8 is: {}", 232 as i8);

    // Since Rust 1.45 the `as` keyword performs a *saturating cast*
    // when casting from float to int. If the floating point value exceeds
    // the upper bound or is less than the lower bound, the returned value
    // will be equal to the bound crossed.

    // 300.0 as u8 is 255. Cannot cross the max.
    println!("300.0 as u8 is: {}", 300.0f32 as u8);
    println!("600.0 as u8 is: {}", 600.0f32 as u8); 

    // And lower. -100.0 as u8 is 0.
    println!("-100.0 as u8 is: {}", -100.0f32 as u8);
    // Let's see what happens as a signed!
    println!("-100.0 as i8 is: {}", -100.0f32 as i8); // -100, well duh.
    println!("-300.0 as i8 is: {}", -300.0f32 as i8); // -128 behold the floor.

    // nan as u8 is 0
    println!("nan as u8 is: {}", f32::NAN as u8);

    // This behavior incurs a small runtime cost and can be avoided with 
    // unsafe methods, however the results might overflow and return
    // **unsound values**. Use these wisely:
    unsafe {
        // 300.0 as u8 is 44.
        println!("300.0 as u8 is: {}", 300.0f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156. Add the paranthesis to clarify that it's a number.
        println!("-100.0 as u8 is: {}", (-100.0f32).to_int_unchecked::<u8>());
        // nan as u8 is 0.
        println!("nan as u8 is: {}", f32::NAN.to_int_unchecked::<u8>());
    }


}