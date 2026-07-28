use std::time::Duration;
use tokio::sync::mpsc;
use tokio::{select, task};

#[tokio::main]
async fn main(){
    let (tx,mut rx) = mpsc::channel(10);
    let (shutdown_tx,mut shutdown_rx) = mpsc::channel::<()>(1);

    let producer = task::spawn( async move {
        loop {
            tx.send(1).await.unwrap();
            tokio::time::sleep(Duration::from_millis(200)).await;
        }
    });

    let cancel_task = task::spawn(async move{
        tokio::time::sleep(Duration::from_millis(350)).await;
        shutdown_tx.send(()).await.unwrap();
    });

    let task = task::spawn(async move{
        loop{
            select! {
             result  = tokio::time::timeout(Duration::from_millis(150), rx.recv()) =>  {
                    match result {
                        Ok(result) => println!("msg: {:?}", result.unwrap() ),
                         Err(_)     => println!("timed out"),
                     }
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