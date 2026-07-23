use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<i32>(9);
    let rx = Arc::new(Mutex::new(rx));
    let rx1 = rx.clone();
    let rx2 = rx.clone();
    async fn consumer(id: i32, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
        loop {
            let msg = rx.lock().await.recv().await;
            match msg {
                Some(msg) => println!("id {}: msg {}", id,msg),
                None => break,
            }
        }
    }

    let task = tokio::spawn(async move {
        tx.send(1).await;
        tx.send(2).await;
        tx.send(3).await;
        tx.send(4).await;
       tx.send(5).await;
        tx.send(6).await;
        tx.send(7).await;
        tx.send(8).await;
        tx.send(9).await;
    });
    let consumer_one = tokio::spawn(async move {
        consumer(1, rx).await;
    });
    let consumer_two = tokio::spawn(async move {
        consumer(2, rx1).await;
    });
    let consumer_three = tokio::spawn(async move {
        consumer(3, rx2).await;
    });
    task.await.unwrap();
    consumer_one.await.unwrap();
    consumer_two.await.unwrap();
    consumer_three.await.unwrap();
}
