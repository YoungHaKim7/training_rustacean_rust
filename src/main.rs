

fn main() {
    static mut FOO: [i32; 5] = [1, 2, 3, 4, 5];

    unsafe {
        println!("{FOO:?}")
    }
    
}
