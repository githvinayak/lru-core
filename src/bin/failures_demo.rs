use tokio::sync::mpsc;
use tokio::task;

#[tokio::main]
async fn main() {
 let (tx,mut rx) = mpsc::channel::<i32>(5);
    let producer = task::spawn(async move{
        for i in 0..=2{
            tx.send(i).await.unwrap();
        }
        panic!("something went wrong");
    });

    loop {
        match rx.recv().await {
            Some(msg) => println!("{:?}",msg),
            None => break
        }
    }

    match producer.await {
        Ok(_)  => println!("task completed"),
        Err(e) => println!("task panicked: {}", e),
    }
}