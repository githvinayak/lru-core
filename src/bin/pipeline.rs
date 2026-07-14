use std::time::Duration;
use tokio::sync::mpsc;
use std::time::Instant;

#[tokio::main]
async fn main() {
    let (tx,mut rx) = mpsc::channel::<i32>(2);
    let tx1 = tx.clone();
    let tx2 = tx.clone();

    async fn producer(tx: mpsc::Sender<i32>) {
        for i in 0..3 {
            println!("before");
            tx.send(i).await.unwrap();
            println!("after");
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    }
    async fn consumer(mut rx:  mpsc::Receiver<i32>) {
        for i in 0..9 {
            let msg = rx.recv().await.unwrap();
            tokio::time::sleep(Duration::from_millis(100)).await;
            println!("msg: {}", msg);
        }
    }
    let producer_start = Instant::now();
   let producer_task_1 = tokio::spawn(async move {
       producer(tx1).await;
   });
    let producer_task_2 = tokio::spawn(async move {
        producer(tx2).await;
    });
    let producer_task_3 = tokio::spawn(async move {
        producer(tx).await;
    });
    let consumer_start = Instant::now();
    let consumer_task = tokio::spawn(async move {
        consumer(rx).await
    });
    producer_task_1.await.unwrap();
    producer_task_2.await.unwrap();
    producer_task_3.await.unwrap();
    println!("producer task done in {:?}", producer_start.elapsed());
    consumer_task.await.unwrap();
    println!("consumer task done in {:?}", consumer_start.elapsed());
}

