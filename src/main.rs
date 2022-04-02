// Atomic Reference Counting with Arc<T>
// Fortunately, Arc<T> is type like "Rc<T>" that safe to use in concurrent situations.
// Atomic Rc

use std::sync::{Arc, Mutex};
use std::thread;


fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();

        *num += 1;
    });
        handles.push(handle)
    }
    
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
