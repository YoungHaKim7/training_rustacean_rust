use rayon::prelude::*;

fn main() {
    // parallel foreach .
    let mut arr = [0, 7, 9, 11];
    arr.par_iter_mut().for_each(|p| {
        let id = std::thread::current().id();
        println!("thread id : {id:?}");
        *p += 1;
    });

    // parallel any/all.
    assert!(arr.par_iter().any(|n| (*n % 2) != 0)); //don't throw panic
    assert!(arr.par_iter().all(|n| (*n >= 1))); //don't throw panic

    println!("rayon let's go !!!");
}

