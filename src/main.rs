fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = v1.clone();
    let v3 = v1.clone();

    println!("{} {} {}", v1.len(), v2.len(), v3.len());
}
