// Enums: types which have a few definite values

enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(m: Movement) {
    // Perform action dependent on info
    match m {
        Movement::Up => println!("Moving Up"),
        Movement::Down => println!("Moving Down"),
        Movement::Left => println!("Moving Left"),
        Movement::Right => println!("Moving Right"),
    }
}

pub fn run() {
    move_avatar(Movement::Up);
    move_avatar(Movement::Down);
    move_avatar(Movement::Left);
    move_avatar(Movement::Right);
}
