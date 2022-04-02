// Mutex is an abbreviation for mutual exclusion, an in,
// a mutex allows only one thread to access some data at any given time.

use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);
{
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {m:?}");
    
}
