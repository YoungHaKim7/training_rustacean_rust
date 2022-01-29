use std::mem::size_of_val; // preprocessing  C 언어

fn main() {
    let x:u8 = 24;
    let y:i32 = 100;

    println!("size is x = {} size is y = {}", size_of_val(&x), size_of_val(&y));
}