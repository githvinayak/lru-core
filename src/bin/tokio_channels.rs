use tokio::sync::mpsc;
#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<i32>(10);
    let task = tokio::task::spawn(async move {
        tx.send(1).await;
        tx.send(1).await;
        tx.send(1).await;
    });

    for i in 0..3{
        let msg = rx.recv().await.unwrap();
        println!("{}",msg);
    }
    task.await.unwrap();
}