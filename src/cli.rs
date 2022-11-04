use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    // 1st argument will be the target of the executable
    println!("Args: {:?}", args);

    let command = args[1].clone();
    println!("Command: {}", command);

    let name = "Nahem";
    let status = "100%";
    if command == "hello" {
        println!("Hi! {}, how are you?", name);
    } else if command == "status" {
        println!("Status: {}", status);
    } else {
        println!("Invalid command!");
    }
}
