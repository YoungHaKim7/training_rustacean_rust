// Using indexing syntax or the "get" method to access an item ina vector
fn main () {
    let v = vec![1,2,3,4,5];

    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);

    println!("does_not_exist : {:?}", does_not_exist);
}
