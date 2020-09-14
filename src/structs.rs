//
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

//tupple
struct Colort(u8, u8, u8);

//
struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }
    fn full_name(&self) -> String {
        return format!("{} {}", self.first_name, self.last_name);
    }

    fn to_tuple(self) -> (String, String) {
        return (self.first_name, self.last_name);
    }
}

pub fn run() {
    let color = Color {
        red: 255,
        green: 0,
        blue: 0,
    };
    println!("{} {} {}", color.blue, color.green, color.red);
    let colort = Colort(0, 255, 0);
    println!("{} {} {}", colort.0, colort.1, colort.2);
    //

    let person = Person::new("Jhon", "Dow");

    println!("Person: {}", person.full_name());
    println!("Person: {:?}", person.to_tuple());
}
