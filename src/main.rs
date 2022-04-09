// struct PointI32 {
//     x: i32,
//     y: i32,
// }
// struct PointF64 {
//     x: f64,
//     y: f64,
// }
// Rust Generics

struct Point<T, U> {
    x: T,
    y: U,
}

struct DougsData<i32, T, U, char> {
    x: i32,
    y: T,
    z: U,
    some_char: char,
}

fn main() {
    // Difference Types
    let a = Point { x: 100, y: -1_f32 };
    println!("x = {}  y = {}", a.x, a.y);

    let b = Point { x: 10.1, y: -2.3 };
    println!("x = {}  y = {}", b.x, b.y);

    let c = DougsData {
        x: 30,
        y: "sting".to_string(),
        z: 40.606060,
        some_char: 'a',
    };
    println!(
        "x = {}\n y = {}\n z = {}\n  some_char = {}",
        c.x, c.y, c.z, c.some_char
    );
}
