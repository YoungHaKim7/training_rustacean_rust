// use std::str;

// fn main() {
//     for b in "안녕 ".bytes() {
//         println!("{}", b);
//     }
//     let a = str::from_utf8(&a).unwrap();
//     println!("{a:?}");
//     let mut my_vec = Vec::new();
//     for my_vec in "하세요 저는 영 ".bytes() {
//         vec.append(my_vec);
//     }
//     let my_vec = str::from_utf8(&my_vec).unwrap();
//     println!("{my_vec}");
// }
fn main() {
    let mut vec = vec![1, 2, 3];
    let mut my_vec2 = vec![236, 149, 136, 235, 133, 149, 32];
    vec.append(&mut my_vec2);
    println!("{vec:?}");
}
