fn main() {
    
    let mut v = [-5i32, 4, 1, -3, 2];

    v.sort_by_cased_key(|k| k.abs());
    assert!(v == [1, 2, -3, 4, -5]);
}
