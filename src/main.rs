// Deref coercion allows Rust to handle these conversions for us automatically.
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}");
}

fn main () {
    let m = MyBox("Rust");
    hello(&(*m)[..]);
}
