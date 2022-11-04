pub fn run() {
    // Print to console
    println!("Hello from the print.rs file!");

    // {} is called placeholder
    // Basic Formatting
    println!("Number: {}", 1);
    println!("{} is from {}", "Nahem", "Space");

    // Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Ramon", "Space", "code"
    );

    // Named Arguments
    println!(
        "{name} likes to play {activity}",
        name = "Charlie",
        activity = "football"
    );

    // Placeholder Traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder For Degug Traits
    println!("{:?}", (12, true, "hola"));

    // Basic Math
    println!("10 + 10 = {}", 10 + 10);
}
