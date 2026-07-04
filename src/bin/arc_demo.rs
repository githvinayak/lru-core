use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let count = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 1..6{
        let clone = count.clone();
        let thread  = thread::spawn(move || {
            let mut val  = clone.lock().unwrap();
            *val += 1;
        });
        handles.push(thread);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("count {}", count.lock().unwrap());
}