// Variables hold primitive data or references to data
// Variables are immutable by default -> you can't reassign them
// Rust is a block-scoped language -> if you set a variable in a function, it pertains to that scope

pub fn run() {
    // Define a Variable
    let name = "Nahem";
    let mut age = 34;
    println!("My name is {} and I am {}", name, age);
    age = 35;
    println!("My name is {} and I am {}", name, age);

    // Define a Constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign Multiple Variables at Once
    let (my_name, my_age) = ("Nahem", 34);
    println!("{} is {}", my_name, my_age);
}
