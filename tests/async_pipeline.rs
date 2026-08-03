
#[cfg(test)]
mod tests{
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::sync::Mutex;
    use tokio::sync::mpsc;
    use lru_core::pipeline::{run_producer, run_worker};

    #[tokio::test]
    async fn test_all_message_processed(){
    let (tx,rx) = mpsc::channel::<i32>(3);
        let rx = Arc::new(Mutex::new(rx));
       let producer =  tokio::spawn(run_producer(tx, vec![1, 2, 3]));
       let task = tokio::spawn(run_worker(1,rx));
        producer.await.unwrap();
        let count = task.await.unwrap();
        assert_eq!(count,3);
    }
    #[tokio::test]
    async fn test_graceful_shutdown(){
        let (tx,rx) = mpsc::channel::<i32>(3);
        let rx = Arc::new(Mutex::new(rx));
        let producer =  tokio::spawn(run_producer(tx, vec![]));
        let task = tokio::spawn(run_worker(1,rx));
        producer.await.unwrap();
        let count = task.await.unwrap();
        assert_eq!(count,0);
    }
    #[tokio::test]
    async fn test_timeout_no_crash() {
        let (tx, rx) = mpsc::channel::<i32>(3);
        let rx = Arc::new(Mutex::new(rx));

        let producer = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(200)).await;
            tx.send(1).await.unwrap();
        });

        let worker = tokio::spawn(run_worker(1, rx));
        producer.await.unwrap();
        let count = worker.await.unwrap();
        assert_eq!(count, 1);
    }
}