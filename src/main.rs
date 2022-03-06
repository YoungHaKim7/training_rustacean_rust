// The generic "Option<T>" is replaced with the specific definitios created by the compiler:
//

#[derive(Debug)]
enum Option_i32 {
    Some(i32),
    None,
}

#[derive(Debug)]
enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);

    println!("integer Option_i32 is : {:?}", integer);
    println!("float Option_f64 is : {:?}", float);
}
