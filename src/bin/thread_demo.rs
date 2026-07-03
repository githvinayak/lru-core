use std::thread;
fn main(){
    let mut handles = vec![];
    for i in 1..4 {
        let thread = thread::spawn(move || {
            println!("hello from thread {}",i);
        });
        handles.push(thread)
    }

    for handle in handles{
        handle.join().unwrap();
    }
}