// struct PointI32 {
//     x: i32,
//     y: i32,
// }
// struct PointF64 {
//     x: f64,
//     y: f64,
// }
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
