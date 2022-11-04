// Functions: used to store blocks of code for re-use

pub fn run() {
    greeting("Hey", "Nahem");

    // Bind functions values to variables
    let get_sum = add(5, 3);
    println!("Sum: {}", get_sum);

    // Closure -> we can use outside variables which can't do with standard functions because it's blocked scope
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure sum: {}", add_nums(11, 12));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}
