use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];

    for i in 1..4{
        let clone = tx.clone();
        let thread  = thread::spawn(move || {
            clone.send(i).unwrap()
        });
        handles.push(thread);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    for i in 0..3{
        println!("{}",rx.recv().unwrap());
    }

}