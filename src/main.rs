// struct PointI32 {
//     x: i32,
//     y: i32,
// }
// struct PointF64 {
//     x: f64,
//     y: f64,
// }
// Rust Generics
// Why use generics
// 1. Abstract Types (aka placeholder types)
// 2. Adds Flexibility
// 3. Reduces Code Duplication
// 4. No runtime cost
// =--------
// 1) Better abstract naming conventions

// 2) Flexible use downstream
// All done at Compile time, not Runtime
// so No runtime cost for using generics

struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let a = Point { x: 100, y: -1 };
    println!("x = {}  y = {}", a.x, a.y);

    let b = Point { x: 10.1, y: -2.3 };
    println!("x = {}  y = {}", b.x, b.y);
}
