// This enum is "Option<T>", and it is defined by the standard library as follows:
//

enum Option<T> {
    None,
    Some(T),
}

fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    println!("{some_number:?}");
}

