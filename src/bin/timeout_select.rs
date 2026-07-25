use std::time::Duration;
use tokio::sync::mpsc;

#[tokio::main]
async fn main(){
    let (tx,mut rx) = mpsc::channel::<i32>(5);

    let producer = tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(200)).await;
        tx.send(2).await.unwrap();
    });

    tokio::select! {
    msg = rx.recv() => {
            println!("msg :{:?}",msg.unwrap())
        }
    _ = tokio::time::sleep(Duration::from_millis(300)) => {
            println!("timeout");
        }
    }

    producer.await.unwrap();
}