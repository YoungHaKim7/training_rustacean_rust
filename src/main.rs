pub trait MyTrait {
    fn sum(&self) -> u32;
}

struct MyStruct {}
impl MyTrait for MyStruct {
    fn sum(&self) -> u32 {
        10
    }
}

fn main() {
    let my_struct = MyStruct {};

    println!("sum: {}", my_struct.sum());
}
