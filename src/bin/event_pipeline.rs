use tokio::sync::Mutex;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;


struct Message {
    producer_id: i32,
    message: i32,
}
#[tokio::main]
async fn main(){
    let (tx,mut rx) = mpsc::channel::<Message>(5);
    let tx1  = tx.clone();
    let tx2  = tx.clone();
    let rx =  Arc::new(Mutex::new(rx));
    let rx1  = rx.clone();
    let rx2  = rx.clone();
    async fn producer(id:i32,tx: mpsc::Sender<Message>){
        for i in 0..3 {
            tx.send(Message{producer_id : id,message:i}).await.unwrap();
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    }
    async fn consumer(id:i32,rx: Arc<Mutex<mpsc::Receiver<Message>>>) {
        loop{
            let msg  = rx.lock().await.recv().await;
            match msg {
                Some(msg) => {
                    println!("id of consumer {}: id of producer{}: msg {}", id,msg.producer_id,msg.message);
                }
                None => break
            }
        }
    }

    let task1 = tokio::spawn( async move {
        producer(1,tx).await;
    });
    let task2 = tokio::spawn( async move {
        producer(2,tx1).await;
    });
    let task3 = tokio::spawn( async move {
        producer(3,tx2).await;
    });

    let worker1 = tokio::spawn( async move {
        consumer(1,rx1).await;
    });
    let worker2 = tokio::spawn( async move {
        consumer(2,rx2).await;
    });
    let worker3 = tokio::spawn( async move {
        consumer(3,rx).await;
    });

    let start  = Instant::now();
    task1.await.unwrap();
    task2.await.unwrap();
    task3.await.unwrap();
    worker1.await.unwrap();
    worker2.await.unwrap();
    worker3.await.unwrap();
    println!("time taken: {:?}", start.elapsed());
}