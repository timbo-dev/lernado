fn main() {

    // Integer Types
    // 8-Bit  i8 
    // 16-Bit i16
    // 32-Bit i32
    // 64-Bit i64
    // 128-Bit i128
    // arch isize

    // the isize will get the computer architecture and define 32-Bit or 64-Bit

    // Unsigned Integer Types
    // 8-Bit  u8 
    // 16-Bit u16
    // 32-Bit u32
    // 64-Bit u64
    // 128-Bit u128
    // arch usize

    let common_decimal: i32 = 98222;
    let decimal_with_underscore: i32 = 98_222;
    let hex: i32 = 0xff;
    let octal: i32 = 0o77;
    let common_binary: i32 = 0b11110000;
    let binary_with_underscore: i32 = 0b1111_0000;
    let byte: u8 = b'A'; // Equal to char of C

    println!("");
    println!("Integers");
    println!("");
    println!("Decimal: {}", common_decimal);
    println!("Decimal with underscore: {}", decimal_with_underscore);
    println!("Hex: {}", hex);
    println!("Octal: {}", octal);
    println!("Binary: {}", common_binary);
    println!("Binary with underscore: {}", binary_with_underscore);
    println!("Byte: {}", byte);

}
