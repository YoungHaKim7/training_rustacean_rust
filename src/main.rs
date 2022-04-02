// Attempting to use "val" after we've sent it down the channel

use std::{sync::mpsc, thread};

fn main() {
    // tx = transmitter
    // rx = receiver
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        println!("val is {val}");
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
}
