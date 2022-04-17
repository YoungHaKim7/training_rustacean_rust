use std::str;

fn main() {
    for b in "안녕 ".bytes() {
        println!("{}", b);
    }
    let a = vec![236, 149, 136, 235, 133, 149, 32];
    let a = str::from_utf8(&a).unwrap();
    println!("{a}");
}
