fn main() {
    println!("1 + 2 = {}", 1u32 + 2);

    println!("1 + 2 = {}", 1i32 - 2);
    // Try changing the above line to 1u32 to see the overflow error.

    // // Scientific notation
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    // // Boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << Xi is {}, {}, {}, {}, {}, {}", 
        1u32 << 1,  1u32 << 2,  1u32 << 3,  1u32 << 4,  1u32 << 5,  1u32 << 6);
    println!("0x80 >> Xi is 0x{:x}, 0x{:x}, 0x{:x}, 0x{:x}, 0x{:x}, 0x{:x}",
        0x80u32 >> 1, 0x80u32 >> 2, 0x80u32 >> 3, 0x80u32 >> 4, 0x80u32 >> 5, 0x80u32 >> 6);

    // Use underscores to improve readability.
    println!("One million is written as {}", 1_000_000u32);
}