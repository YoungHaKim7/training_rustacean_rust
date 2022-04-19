use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    let my_name = "Billybrobby";
    let my_country = "USA";
    let my_home = "Korea";

    println!("{}", type_of(my_name));
    let together = format!(
        "I am {} and I come from {} but I live in {}.",
        my_name, my_country, my_home
    );
    let my_string: String = "Try to make this a String".into();
    println!("{my_string}");
}

