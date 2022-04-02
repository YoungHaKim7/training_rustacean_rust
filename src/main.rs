// Creating a new thread to print one thing while the main thread prints something else

use std::time::Duration;
use std::thread;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} form the spawned thread!");
            thread::sleep(Duration::from_millis(1));

    }});

    for i in 1..5 {
        println!("hi number {i}");
        thread::sleep(Duration::from_millis(1));
    }
}

