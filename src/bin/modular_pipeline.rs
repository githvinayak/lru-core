use tokio::sync::Mutex;
use std::sync::Arc;
use std::time::{Instant,Duration};
use tokio::sync::mpsc;
use tokio_stream::StreamExt;



async fn run_producer(tx:mpsc::Sender<i32>,items:Vec<i32>){
        let mut stream = tokio_stream::iter(items);
        while let Some(val) = stream.next().await{
            let start = Instant::now();
            println!("sending {} at {:?}", val, start.elapsed());
            tx.send(val).await.unwrap();
            println!("sent {} at {}ms", val, start.elapsed().as_millis());
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
 }
async fn run_worker(id:i32,rx:Arc<Mutex<mpsc::Receiver<i32>>>,start: Arc<Instant>){
        loop{
            let msg  = rx.lock().await.recv().await;
            println!("worker {} received {:?} at {}ms, processing...",id, msg, start.elapsed().as_millis());
            tokio::time::sleep(Duration::from_millis(300)).await;
            println!("worker {} finished {:?} at {}ms",id, msg, start.elapsed().as_millis());
            match msg {
                Some(msg) => {
                    println!(" msg {}",msg);
                }
                None => break
            }

        }
}

async fn run_pipeline(){
    let (tx,rx) = mpsc::channel::<i32>(3);
    let rx = Arc::new(Mutex::new(rx));
    let rx1 = rx.clone();
    let start  = Arc::new((Instant::now()));
    let start1 = start.clone();
    let mut handles = Vec::new();

    handles.push(tokio::spawn(run_producer(tx,vec![1, 2, 3,4,5,6,7,8,9,10])));
    handles.push(tokio::spawn(run_worker(1,rx,start)));
    handles.push(tokio::spawn(run_worker(2,rx1,start1)));

    for handle in handles{
        handle.await.unwrap();
    }

}
#[tokio::main]
async fn main() {
    run_pipeline().await;
}