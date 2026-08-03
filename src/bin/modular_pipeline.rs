use tokio::sync::Mutex;
use std::sync::Arc;
use std::time::{Instant,Duration};
use tokio::sync::mpsc;
use tokio_stream::StreamExt;

const CHANNEL_CAPACITY: usize = 3;

const PRODUCER_PROCESS_MS: u64 = 10;
const WORKER_PROCESS_MS: u64 = 300;
const WORKER_TIMEOUT_MS: u64 = 150;

async fn run_producer(tx:mpsc::Sender<i32>,items:Vec<i32>){
        let mut stream = tokio_stream::iter(items);
        while let Some(val) = stream.next().await{
            tx.send(val).await.unwrap();
            tokio::time::sleep(Duration::from_millis(PRODUCER_PROCESS_MS)).await;
        }
 }
async fn run_worker(id:i32,rx:Arc<Mutex<mpsc::Receiver<i32>>>){
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
}

async fn run_pipeline(){
    let (tx,rx) = mpsc::channel::<i32>(CHANNEL_CAPACITY);
    let rx = Arc::new(Mutex::new(rx));
    let rx1 = rx.clone();
    let start_time = Instant::now();
    let mut handles = Vec::new();

    handles.push(tokio::spawn(run_producer(tx,vec![1, 2, 3,4,5,6,7,8,9,10])));
    handles.push(tokio::spawn(run_worker(1,rx)));
    handles.push(tokio::spawn(run_worker(2,rx1)));

    for handle in handles{
        handle.await.unwrap();
    }
    println!("total time: {:?}", start_time.elapsed());

}
#[tokio::main]
async fn main() {
    run_pipeline().await;
}