// Constraint == Gernerics ___canot add 'T' to 'T'
fn dougs_func<T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::fmt::Debug>(
    input_a: T,
    input_b: T,
) -> T {
    input_a - input_b
}

fn main() {
    let a = dougs_func(4.4, 5.5);
    println!("a has {a}")
}
