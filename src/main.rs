fn greet(s: &String) {
    println!("Hello, {} !", s);
}

fn main() {
    let name = String::from("Young");
    greet(&name);
    println!("Hello again, {}", name);
}

