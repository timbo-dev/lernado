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

    // By default rust use f64 for float numbers

    let float_64_bits = 2.0;
    let float_32_bits: f32 = 2.0;

    let float_32_bits_precision: f32 = 0.1 + 0.2;
    let float_32_bits_parameter: f32 = 0.3;

    println!("");
    println!("Floats");
    println!("");
    println!("Float f64: {}", float_64_bits);
    println!("Float f32: {}", float_32_bits);
    println!("Equality of floats type: float_64_bits == float_32_bits -> {}", float_64_bits == float_32_bits);
    println!("Float f64 precision equality: 0.1 + 0.2 == 0.3 -> {}", 0.1 + 0.2 == 0.3);
    // I don't know yet but f32 solves the little mess of approximate numbers to this calculation
    println!("Float f32 precision equality: 0.1 + 0.2 == 0.3 -> {}", float_32_bits_precision == float_32_bits_parameter);

    // Numeric Operations

    println!("");
    println!("Numeric Operations");
    println!("");

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    let remainder = 43 % 5;

    println!("Sum: {}", sum); // sum
    println!("Difference: {}", difference); // subtraction
    println!("Product: {}", product); // multiplication
    println!("Quotient: {}", quotient); // division
    println!("Truncated: {}", truncated);
    println!("Remainder: {}", remainder); // remainder

    // Boolean

    println!("");
    println!("Boolean");
    println!("");

    let true_boolean: bool = true;
    let false_boolean: bool = false;
    
    println!("True Boolean: {}", true_boolean);
    println!("False Boolean: {}", false_boolean);
    
    // Char

    println!("");
    println!("Char");
    println!("");

    let char_value: char = 'z';
    let more_one_byte_char: char = 'ℤ';
    let emoji_char: char = '😻';

    println!("Char Value: {}", char_value);
    println!("More One Byte Char: {}", more_one_byte_char);
    println!("Emoji Char: {}", emoji_char);

    // Compund types

    // Tuple type

    println!("");
    println!("Tuple");
    println!("");

    let tuple: (i32, f64, u8) = (500, 6.4, 1);

    let (first, second, third) = tuple;

    println!("Desctructured tuple first: {first}");
    println!("Desctructured tuple second: {second}");
    println!("Desctructured tuple third: {third}");

    println!("Access tuple from dot 0: {}", tuple.0);
    println!("Access tuple from dot 1: {}", tuple.1);
    println!("Access tuple from dot 2: {}", tuple.2);

    // Array type

    println!("");
    println!("Array");
    println!("");

    let array = [1, 2, 3, 4, 5];
    
    let mut i = 0;

    while i < array.len() {
        println!("Array element [{i}]: {}", array[i]);
        i += 1;
    }

    let months = [
        "January", 
        "February", 
        "March", 
        "April", 
        "May", 
        "June", 
        "July", 
        "August", 
        "September", 
        "October", 
        "November", 
        "December"
    ];

    i = 0;
    
    println!("");

    while i < months.len() {
        println!("Array element [{i}]: {}", months[i]);
        i += 1;
    }

    // let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];
        
    println!("Array 'a' [{},{},{},{},{}]", a[0], a[1], a[2], a[3], a[4]);
}
