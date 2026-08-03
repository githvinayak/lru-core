use tokio::sync::Mutex;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::mpsc;
use lru_core::pipeline::{run_worker,run_producer};

const CHANNEL_CAPACITY: usize = 3;

async fn run_pipeline(){
    let (tx,rx) = mpsc::channel::<i32>(CHANNEL_CAPACITY);
    let rx = Arc::new(Mutex::new(rx));
    let rx1 = rx.clone();
    let start_time = Instant::now();
    let mut producer_handles = Vec::new();
    let mut worker_handles = Vec::new();

    producer_handles.push(tokio::spawn(run_producer(tx,vec![1, 2, 3,4,5,6,7,8,9,10])));
    worker_handles.push(tokio::spawn(run_worker(1,rx)));
    worker_handles.push(tokio::spawn(run_worker(2,rx1)));

    for handle in producer_handles{
        handle.await.unwrap();
    }
    for handle in worker_handles{
        handle.await.unwrap();
    }
    println!("total time: {:?}", start_time.elapsed());

}
#[tokio::main]
async fn main() {
    run_pipeline().await;
}