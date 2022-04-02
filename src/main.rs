// Using Message Passing to Transfer Data Between Threads

use std::{sync::mpsc, thread};

fn main() {
    // tx = transmitter
    // rx = receiver
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
}
