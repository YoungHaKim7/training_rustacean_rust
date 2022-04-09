struct PointI32 {
    x: i32,
    y: i32,
}
struct PointF64 {
    x: f64,
    y: f64,
}

fn main() {
    let a = PointI32 { x: 100, y: -1};
    println!("x = {}  y = {}", a.x, a.y);
}
