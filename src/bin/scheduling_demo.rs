use std::time::Duration;

#[tokio::main]
async fn main() {
    let taskA = tokio::spawn(async {
        for i in 1..=3{
            println!("A working");
            tokio::time::sleep(Duration::from_millis(100)).await;
            println!("A done");
        }
    });
    let taskB  = tokio::spawn(async {
        for i in 1..=3{
            println!("B working");
            tokio::time::sleep(Duration::from_millis(100)).await;
            println!("B done");
        }
    });
     taskA.await.unwrap();
    taskB.await.unwrap();
}