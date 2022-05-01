fn main() {
    let mut v = vec![];
    v.push("hello");

    let hello = &v[0];
    println!("{hello}");

    v.push("Rust");
    println!("{hello}");
}
