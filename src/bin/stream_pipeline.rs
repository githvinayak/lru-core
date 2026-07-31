use tokio::sync::Mutex;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let (tx,mut rx) = mpsc::channel::<i32>(3);
    let rx = Arc::new(Mutex::new(rx));
    let rx1 = rx.clone();
    async fn consumer(mut rx:Arc<Mutex<mpsc::Receiver<i32>>>) {
        loop{
            let msg  = rx.lock().await.recv().await;
            match msg {
                Some(msg) => {
                    println!(" msg {}",msg);
                }
                None => break
            }
            tokio::time::sleep(Duration::from_millis(150)).await;
        }
    }

    let task = tokio::spawn(async move {
        let mut stream = tokio_stream::iter(vec![1, 2, 3,4,5,6,7,8,9,10]);
        while let Some(val) = stream.next().await{
            tx.send(val).await.unwrap();
        }
    });

    let worker_one = tokio::spawn(async move {
        consumer(rx).await;
    });

    let worker_two = tokio::spawn(async move {
        consumer(rx1).await;
    });

    task.await.unwrap();
    worker_one.await.unwrap();
    worker_two.await.unwrap();
}