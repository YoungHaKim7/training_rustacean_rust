use std::mem::size_of_val;

fn main() {
    let a: char = 'a';
    let b: u8 = 1;
    let c: i16 = 2;
    let d: i32 = 3;
    let e: i64 = 4;
    let f: i128 = 5;
    let g: f32 = 6.;
    let h: f64 = 7.;

    println!("char size of val: {}", size_of_val(&a));
    println!("u8 size of val: {}", size_of_val(&b));
    println!("i16 size of val: {}", size_of_val(&c));
    println!("i32 size of val: {}", size_of_val(&d));
    println!("i64 size of val: {}", size_of_val(&e));
    println!("i128 size of val: {}", size_of_val(&f));
    println!("float __32bit : {}", size_of_val(&g));
    println!("float __64bit : {}", size_of_val(&h));
}
