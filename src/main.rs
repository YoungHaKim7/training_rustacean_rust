fn greet(s: String) {
    println!("Hello, {s} !");
}

fn main() {
    let name = String::from("Pavel");
    greet(name);
    println!("Hello again, {name}");
}

