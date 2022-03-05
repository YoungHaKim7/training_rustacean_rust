// Shortcuts for Panic or Error: unwrap and expect
//
use std::fs::File; 

fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
