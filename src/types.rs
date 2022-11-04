/*
Primitive Types:
    Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
    Floats: f32, f64
    Boolean (bool): true, false
    Characters (char): 'a' -> one character only
    Tuples: (1, "a")
    Arrays: [0, 1, 2, 3] -> Fixed lenght
    (non primitive) Vectors -> growable arrays
*/

// Rust is a statically typed language, which means that it must know the types of all
// variables at compile time. However, the compiler can usually infer what type we want
// to use based on the value and how to use it

pub fn run() {
    // Default for int is i32
    let x = 1;

    // Default for float is f64
    let y = 2.4;

    // Add explicit  type
    let z: i64 = 555555555;

    // Find max size
    println!("Max i32: {}", i32::MAX);
    println!("Max i64: {}", i64::MAX);

    // Boolean
    let is_active = true;
    println!("{:?}", (x, y, z, is_active));

    // Get boolean from expression
    let is_greater = 10 > 5;
    println!("10 is greater than 5? {}", is_greater);

    // Char
    let a = 'a';
    println!("{}", a);

    // Unicode for emoji
    let moon = '\u{1F680}';
    println!("We're going... {}", moon);
}
