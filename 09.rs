fn main() {
    // Addition
    println!("1 + 2 = {}", 1u32 + 2);
    // Subtraction
    println!("1 - 2 = {}", 1i32 - 2);

    // Bool logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("not true is {}", !true);

    // Bitwise
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Check out these fancy underscores
    println!("I have like ${} because I learned unpopular programming languages!", 1_000_000u32);
}