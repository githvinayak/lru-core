use std::time::Duration;
use tokio_stream::StreamExt;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use std::sync::Arc;

const PRODUCER_PROCESS_MS: u64 = 10;
const WORKER_PROCESS_MS: u64 = 300;
const WORKER_TIMEOUT_MS: u64 = 150;

pub async fn run_producer(tx:mpsc::Sender<i32>,items:Vec<i32>){
    let mut stream = tokio_stream::iter(items);
    while let Some(val) = stream.next().await{
        tx.send(val).await.unwrap();
        tokio::time::sleep(Duration::from_millis(PRODUCER_PROCESS_MS)).await;
    }
}
pub async fn run_worker(id:i32,rx:Arc<Mutex<mpsc::Receiver<i32>>>)->i32{
    let mut count:i32 = 0;
    loop{
        let msg =  match tokio::time::timeout(Duration::from_millis(WORKER_TIMEOUT_MS),rx.lock().await.recv()).await {
            Ok(Some(msg)) =>{
                count += 1;
                msg
            }
            Ok(None) => {
                println!("worker {} processed {} messages", id, count);
                break;
            }
            Err(_) =>{
                println!("timeout");
                continue
            }
        };
        tokio::time::sleep(Duration::from_millis(WORKER_PROCESS_MS)).await;
    }
    count
}