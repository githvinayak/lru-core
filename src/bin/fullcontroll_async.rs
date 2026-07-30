use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::Duration;
use tokio::select;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx,mut rx) = mpsc::channel::<i32>(5);
    let (s_tx1, mut s_rx1) = mpsc::channel::<()>(1);
    let (s_tx2,mut  s_rx2) = mpsc::channel::<()>(1);
    let (s_tx3, mut s_rx3) = mpsc::channel::<()>(1);
    let tx1 = tx.clone();
    let tx2 = tx.clone();
    let rx = Arc::new(Mutex::new(rx));
    let rx1 = rx.clone();
    let rx2 = rx.clone();

    async fn producer(mut tx: mpsc::Sender<i32>) {
        loop {
            tx.send(1).await.unwrap();
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }
    // async fn consumer(mut rx: Arc<Mutex<mpsc::Receiver<i32>>>,mut shutdown_rx: mpsc::Receiver<()>) {
    //     loop {
    //         let msg = {
    //             let mut guard = rx.lock().await;
    //             guard.recv().await
    //         };
    //         select! {
    //             result  = tokio::time::timeout(Duration::from_millis(150),msg) => {
    //                 match result {
    //                     Ok(result) => println!("msg: {:?}", result.unwrap() ),
    //                     Err(_) => break,
    //                 }
    //             }
    //             result = shutdown_rx.recv() =>{
    //                 break;
    //             }
    //         }
    //
    //     }
    // }

    async fn consumer(mut rx: Arc<Mutex<mpsc::Receiver<i32>>>,mut shutdown_rx: mpsc::Receiver<()>) {
        loop {
            select! {
                 result =  tokio::time::timeout(Duration::from_millis(150),async { rx.lock().await.recv().await })  => {
            match result {
                Ok(m) => println!("msg {:?}", m.unwrap()),
                Err(_) => println!("timeout"),
            }
               }
                result = shutdown_rx.recv() =>{
                    break;
                }
            }

        }
    }
 let cancel_task = tokio::spawn(async move {
     tokio::time::sleep(Duration::from_millis(550)).await;
     s_tx1.send(()).await.unwrap();
     s_tx2.send(()).await.unwrap();
     s_tx3.send(()).await.unwrap();
 });

    let producer_one = tokio::spawn( async move {
        producer( tx ).await;
    });
    let producer_two = tokio::spawn( async move {
        producer( tx1 ).await;
    });
    let producer_three = tokio::spawn( async move {
        producer( tx2 ).await;
    });


    let worker_one = tokio::spawn( async move {
        consumer( rx,s_rx1 ).await;
    });
    let worker_two = tokio::spawn( async move {
        consumer( rx1,s_rx2 ).await;
    });
    let worker_three = tokio::spawn( async move {
        consumer( rx2,s_rx3 ).await;
    });
    cancel_task.await.unwrap();

    worker_one.await.unwrap();
    worker_two.await.unwrap();
    worker_three.await.unwrap();
}