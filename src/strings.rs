// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure. Used when you need to modify or own string data

pub fn run() {
    let mut hello = String::from("Hello ");
    println!("{}", hello);

    // Get lenght
    println!("Lenght: {}", hello.len());

    // Push a char
    hello.push('W');
    println!("{}", hello);

    // Push a string
    hello.push_str("orld!");
    println!("{}", hello);

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Check if empty
    println!("Is empty: {}", hello.is_empty());

    // Contains
    println!("Contains the word 'World': {}", hello.contains("World"));

    // Replace
    println!("Replace: {}", hello.replace("World", "There"));

    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}", s);

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
}
