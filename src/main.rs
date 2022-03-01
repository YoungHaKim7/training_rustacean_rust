// This enum is "Option<T>", and it is defined by the standard library as follows:
#[derive(Debug)]
enum Option<T> {
    None,
    Some(T),
}

fn main() {
    let some_number = Option::Some(5);
    let some_string = Option::Some("a string".to_string());

    let absent_number: Option<i32> = Option::None;

    println!("{some_number:?}");
    println!("{some_string:?}");
    println!("{absent_number:?}");
}

