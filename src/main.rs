fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    println!("{}", vec.len());
    println!("{vec:?}");
    let my_vec_pop = vec.pop();
    println!("{my_vec_pop:?}");
    println!("{vec:?}");
    vec.push(2);
    vec.push(3);
    let my_vec_push = vec;
    println!("{my_vec_push:?}");
    println!("{:?}", my_vec_push[2]);
    println!("------------------");

    let mut my_vec_extend = vec![4, 5, 6];
    my_vec_extend.extend([1, 2, 3].iter().copied());

    for x in &my_vec_extend {
        println!("{x}");
    }
    println!("------------------");
    let my_vec2 = vec![0; 5];
    println!("{my_vec2:?}");

    println!("------------------");
    // The following is equivalent, but potentially slower;
    let mut vec3 = Vec::with_capacity(6);
    vec3.resize(7, 3);
    println!("{vec3:?}");

    println!("--// Use a Vec<T> as an effcient stack:--");
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        // Print 3,2,1
        println!("{top}");
    }

    println!("------------------------------");
    println!("Indexing");

    let v = vec![0, 2, 4, 6];
    println!("{}", v[1]);
    // println!("{}", v[5]);
    let u = &v[3];
    println!("{u}");
}
