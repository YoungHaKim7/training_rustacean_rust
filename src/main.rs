// Constraint == Gernerics ___canot add 'T' to 'T'
// fn dougs_func<T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::fmt::Debug>(
//     input_a: T,
//     input_b: T,
// ) -> T {
//     input_a - input_b
// }

#[allow(dead_code)]
fn dougs_func2<T, E>(input_a: T, input_b: T, input_e: E) -> T
where
    T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::fmt::Debug,
    E: std::fmt::Debug,
{
    println!("input_a has {:?}", input_a);
    println!("input_e has {:?}", input_e);
    input_a - input_b
}

// Not about programming to Specific Type
// It's about programming to CAPABILITIES of Types via Constrains on Traits

fn main() {
    let a = dougs_func2(4.4, 5.5, "debug".to_string());
    println!("a has {a}");
}
