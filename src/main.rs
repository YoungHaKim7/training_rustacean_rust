// Creating a new Vector
fn main () {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    {
        let v = vec![1,2,3,4];

        println!("Vector v __ scope in : {v:?}")

        // let z = vec![5,6,7,8,9,10]; // < z goes out of scope and is freed here___drop "z"
        // do stuff with v
    } // <-- v goes out of scope and is freed here

    println!("Vector print: {v:?}");
//    println!("Vector print: {z:?}");
}

