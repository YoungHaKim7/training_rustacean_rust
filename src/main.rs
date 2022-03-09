struct Animal {
    name: String,
}

//usually verbs, or adjectives
trait Canine {
    fn bark(&self) {
        println!("Woof woof");
    }
    fn run(&self) {
        println!("The dog is running");
    }
}

impl Canine for Animal {}

fn main() {
    let rover = Animal {
        name: "Rover".to_string(),
    };

    rover.bark();
    rover.run();
}
