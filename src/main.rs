fn main() {
    let mut number1: i32 = 10;
    let number1_ref = &mut number1;
    let number1_ref_ptr_add = number1_ref.clone();
    *number1_ref = 9;

    println!("Rust \n Refenrece & Deference & pointer Address");
    println!(
        "Number 1: {} \n size of (number1) : {}",
        number1,
        std::mem::size_of_val(&number1)
    );
    println!("Number 1 pointer address : {:p}", &number1_ref_ptr_add);

    let mut float64_02: f64 = 23.456;
    let float64_02_ref = &mut float64_02;
    let float64_02_ref_ptr_add = float64_02_ref.clone();
    *float64_02_ref = 45.789;
    let f64_add = float64_02_ref_ptr_add.to_bits();

    println!("\n Rust \n Refenrece & Deference & pointer Address");
    println!(
        "Number 2: {} \n size of (float64_02) : {}",
        float64_02,
        std::mem::size_of_val(&float64_02)
    );
    println!("Number 2 pointer address : {:p}", &f64_add);
}


