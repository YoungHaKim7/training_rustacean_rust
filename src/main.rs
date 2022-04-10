struct MyStruct {
    a: i32,
    b: i32,
}

impl std::fmt::Display for MyStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.a, self.b)
    }
}

pub trait PrintValue {
    fn print_value(&self);
}

impl<T: std::fmt::Display> PrintValue for T {
    fn print_value(&self) {
        println!("{self}");
    }
}

fn main() {
    let m1 = MyStruct { a: 10, b: 12 };
    m1.print_value();
    let m2 = String::from("Hello");
    m2.print_value();
}
