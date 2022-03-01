// For example, this code won't compile because it's tring to add an "i8" to an "Option<i8>"

enum Option<T> {
    None,
    Some(T),
}

fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;

    println!("{sum:?}");
}

