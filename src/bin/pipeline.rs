use std::time::Duration;
use tokio::sync::mpsc;
use std::time::Instant;

#[tokio::main]
async fn main() {
    let (tx,mut rx) = mpsc::channel::<i32>(5);
    async fn producer(tx: mpsc::Sender<i32>) {
        for i in 0..5 {
            tx.send(i).await.unwrap();
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    }
    async fn consumer(mut rx:  mpsc::Receiver<i32>) {
        for i in 0..5 {

            let msg = rx.recv().await.unwrap();
            tokio::time::sleep(Duration::from_millis(100)).await;
            println!("msg: {}", msg);
        }
    }
    let producder_start = Instant::now();
   let producer_task = tokio::spawn(async move {
       producer(tx).await;
   });
    let consumer_start = Instant::now();
    let consumer_task = tokio::spawn(async move {
        consumer(rx).await
    });
    producer_task.await.unwrap();
    println!("producer task done in {:?}", producder_start.elapsed());
    consumer_task.await.unwrap();
    println!("consumer task done in {:?}", consumer_start.elapsed());
}