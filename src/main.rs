const fn factorial(n: usize) -> usize {
    // prnltln!("Computing factorial of {n}")
    let mut total = 1;
    let mut step = 1;
    loop {
        if step == n {
            break;
        }
        step += 1;
        total *= step;
    }
    return total;
}

const FACTORIAL_OF_10: usize = factorial(10);

fn main() {
    let factorial_of_11: usize = factorial(11);
    println!("Hello, wolrd! {}", FACTORIAL_OF_10);
    println!("Hello, wolrd! {}", factorial_of_11);
}
