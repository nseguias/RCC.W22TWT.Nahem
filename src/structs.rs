// Structs: used to create custom data types -> similar to classes

// Traditional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple struct
struct Color2(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // Construct a person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }
    // Get full name
    fn get_full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
    // Set last name
    fn set_last_name(&mut self, new_last_name: &str) {
        self.last_name = new_last_name.to_string()
    }
    // Name to tuple
    fn name_to_tuple(self) -> (String, String) {
        (self.first_name.to_string(), self.last_name.to_string())
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };
    println!("Color: {} {} {}", c.red, c.green, c.blue);

    c.red = 200;
    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut d = Color2(0, 255, 0);
    println!("Color2: {} {} {}", d.0, d.1, d.2);

    d.2 = 100;
    println!("Color2: {} {} {}", d.0, d.1, d.2);

    let mut p = Person::new("Nahem", "Seguias");
    println!("Person: {} {}", p.first_name, p.last_name);
    println!("Person: {}", p.get_full_name());

    let mut p2 = Person::new("Nancy", "Riley");
    println!("Single: {}", p2.get_full_name());
    p2.set_last_name("Seguias");
    println!("Married: {}", p2.get_full_name());
    println!("Person as tuple: {:?}", p.name_to_tuple());
}
