fn main() {
    let mut my_name = String::with_capacity(30);
    my_name.push_str("String::from, hi Kim");
    println!(
        "Capacity is : {}, lens is : {}",
        my_name.capacity(),
        my_name.len()
    );
    my_name.push_str("String::from, gogogogogo");
    println!(
        "Capacity is : {}, lens is : {}",
        my_name.capacity(),
        my_name.len()
    );
    my_name.push_str("String::from, gogogogogo");

    println!(
        "Capacity is : {}, lens is : {}",
        my_name.capacity(),
        my_name.len()
    );
    my_name
        .push_str("String::from, gogogogogogogogogogogogogogogogogogogogogogogogogogogogogooggo");

    println!(
        "Capacity is : {}, lens is : {}",
        my_name.capacity(),
        my_name.len()
    );
}
