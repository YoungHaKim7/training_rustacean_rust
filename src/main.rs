#[derive(Debug)]
struct Cat {
    name: String,
    age: u8
}

fn main () {
    let mr_mantle = Cat {
        name: "Reggie Mantle".to_string(),
        age: 4
    };
    println!("Reggie Mantle is a {:?}", mr_mantle);
}
