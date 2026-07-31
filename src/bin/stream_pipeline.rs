use tokio::sync::Mutex;
use std::sync::Arc;
use std::time::{Instant,Duration};
use tokio::sync::mpsc;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let (tx,mut rx) = mpsc::channel::<i32>(3);
    let rx = Arc::new(Mutex::new(rx));
    let rx1 = rx.clone();
    let start  = Arc::new(Mutex::new(Instant::now()));
    let start1 = start.clone();
    async fn consumer(id:i32,mut rx:Arc<Mutex<mpsc::Receiver<i32>>>,start:Arc<Mutex<Instant>>) {
        loop{
            let msg  = rx.lock().await.recv().await;
            println!("worker {} received {:?} at {}ms, processing...",id, msg, start.lock().await.elapsed().as_millis());
            tokio::time::sleep(Duration::from_millis(300)).await;
            println!("worker {} finished {:?} at {}ms",id, msg, start.lock().await.elapsed().as_millis());
            match msg {
                Some(msg) => {
                    println!(" msg {}",msg);
                }
                None => break
            }

        }
    }

    let task = tokio::spawn(async move {
        let mut stream = tokio_stream::iter(vec![1, 2, 3,4,5,6,7,8,9,10]);
        while let Some(val) = stream.next().await{
            let start = Instant::now();
            println!("sending {} at {:?}", val, start.elapsed());
            tx.send(val).await.unwrap();
            println!("sent {} at {}ms", val, start.elapsed().as_millis());
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    });

    let worker_one = tokio::spawn(async move {
        consumer(1,rx,start).await;
    });

    let worker_two = tokio::spawn(async move {
        consumer(2,rx1,start1).await;
    });

    task.await.unwrap();
    worker_one.await.unwrap();
    worker_two.await.unwrap();
}