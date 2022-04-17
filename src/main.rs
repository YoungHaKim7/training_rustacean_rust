
fn main() {
    let s: &str = "a";
    let ss: String = s.to_owned();

    let v: &[i32] = &[1, 2];
    let vv: Vec<i32> = v.to_owned();

    println!("{ss}");
    println!("{vv:?}");
}
