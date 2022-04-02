
use std::time::Duration;
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} form the spawned thread!");
            thread::sleep(Duration::from_millis(1));

    }});
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {i}");
        thread::sleep(Duration::from_millis(1));
    }

}

