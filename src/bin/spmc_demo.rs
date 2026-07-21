use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<i32>(9);
    let rx = Arc::new(Mutex::new(rx));
    let rx1 = rx.clone();
    let rx2 = rx.clone();
    let (shutdown_tx1, shutdown_rx1) = mpsc::channel::<()>(1);
    let (shutdown_tx2, shutdown_rx2) = mpsc::channel::<()>(1);
    let (shutdown_tx3, shutdown_rx3) = mpsc::channel::<()>(1);
    async fn consumer(id: i32, rx: Arc<Mutex<mpsc::Receiver<i32>>>,mut shutdown_rx: mpsc::Receiver<()>) {
        loop {
            let mut guard  = rx.lock().await;
            tokio::select! {
                Some(msg) = guard.recv()=> { println!("worker id :{}, rec msg:{}",id, msg);}
             _ = shutdown_rx.recv() => { break; }  // shutdown signal received
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
        consumer(1, rx,shutdown_rx1).await;
    });
    let consumer_two = tokio::spawn(async move {
        consumer(2, rx1,shutdown_rx2).await;
    });
    let consumer_three = tokio::spawn(async move {
        consumer(3, rx2,shutdown_rx3).await;
    });
    task.await.unwrap();
    shutdown_tx1.send(()).await.unwrap();
    shutdown_tx2.send(()).await.unwrap();
    shutdown_tx3.send(()).await.unwrap();
    consumer_one.await.unwrap();
    consumer_two.await.unwrap();
    consumer_three.await.unwrap();
}
