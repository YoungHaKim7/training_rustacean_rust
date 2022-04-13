fn main() {
    let mut my_name = String::with_capacity(26);
    println!(
        "Length is {} and capacity is : {}",
        my_name.len(),
        my_name.capacity()
    );

    my_name.push_str("David!");
    println!(
        "Length is {} and capacity is : {}",
        my_name.len(),
        my_name.capacity()
    );

    my_name.push_str(" and I live in Seoul");
    println!(
        "Length is {} and capacity is : {}",
        my_name.len(),
        my_name.capacity()
    );
}
