// Arrays: Fixed list where elements are the same data type

use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numbers);
    println!("1st value: {}", numbers[0]);

    // Reassign a value
    numbers[2] = 20;
    println!("3rd value: {}", numbers[2]);

    // Get array length
    println!("Array length: {}", numbers.len());

    // Arrays are stack-allocated
    println!("This array occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}
