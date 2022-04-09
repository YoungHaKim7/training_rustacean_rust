fn dougs_func<T>(input_a: T, input_b: T) -> T {
    input_a + input_b
}

fn main() {
    let a = dougs_func(4 as i8, 5);
    println!("a has {a}")
}
