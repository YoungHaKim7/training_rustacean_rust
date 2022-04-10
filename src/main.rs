//default
pub trait MyTrait {
    fn sum(&self) -> u32 {
        10
    }
}

struct MyStruct {}
impl MyTrait for MyStruct {}

struct MyStruct2 {
    size: u32,
}

impl MyTrait for MyStruct2 {
    fn sum(&self) -> u32 {
        self.size
    }
}

fn print_sum(m: &impl MyTrait) {
    println!("sum: {}", m.sum());
}

fn main() {
    let my_struct = MyStruct {};

    print_sum(&my_struct);
}
