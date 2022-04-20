use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    let string_1 = String::from("Hello world");
    let string_2 = "Hello world".to_string();
    let string_3 = "Hello world";
    let string_4: String = "Hello world".into();
    let string_5 = "Hello world".to_owned();

    println!("{}", type_of(string_1));
    println!("{}", type_of(string_2));
    println!("{}", type_of(string_3));
    println!("{}", type_of(string_4));
    println!("{}", type_of(string_5));
}

