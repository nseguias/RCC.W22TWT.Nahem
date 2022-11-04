pub fn run() {
    let age = 22;
    let check_id = true;
    let knows_person_of_age = true;
    // If / Else
    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender: what do you like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: sorry, you have to leave");
    } else {
        println!("Bartender: show me your ID");
    }

    // shorthand if -> similar to a ternary operator
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is of age: {}", is_of_age);
}
