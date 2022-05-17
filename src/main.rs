fn main() {
    let needle = 0o204;
    let haystack = vec![1, 1, 2, 5, 14, 132, 420, 1430];

    if haystack.contains(&needle) {
        println!("{needle}");
    }
}

