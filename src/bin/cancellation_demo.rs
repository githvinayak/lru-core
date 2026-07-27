use std::time::Duration;
use tokio::select;
use tokio::sync::mpsc;


#[tokio::main]
async fn main() {
    let (tx,mut rx) = mpsc::channel::<i32>(5);
    let (shutdown_tx, mut shutdown_rx) = mpsc::channel::<()>(5);

    let producer = tokio::spawn(async move {
        for i in 1..=5 {
            tx.send(i).await.unwrap();
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    });
    let cancel_task = tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(350)).await;
        shutdown_tx.send(()).await.unwrap();
    });
    let task = tokio::spawn(async move {
        loop {
            select! {
               result  = rx.recv() => {
                    println!("rx: {:?}", result.unwrap());
                }
                result = shutdown_rx.recv() => {
                    break;
                }

            }
        }
    });
    cancel_task.await.unwrap();
    task.await.unwrap();
}