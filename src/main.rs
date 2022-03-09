#[derive(Debug)]
struct Cat {
    name: String,
    age: u8,
}

impl std::fmt::Display for Cat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cat_type = match self.age {
            0..=2 => "kitten",
            3..=10 => "adult cat",
            _ => "old cat",
        };
        write!(
            f,
            "{} is a cat who is {} years old, and therefore a {}",
            self.name, self.age, cat_type
        )
    }
}

fn main() {
    let mr_mantle = Cat {
        name: "Reggie Mantle".to_string(),
        age: 4,
    };
    println!(
        "This many characters: {} ",
        mr_mantle.to_string().chars().count()
    );
}
