use std::time::Duration;

async fn r1()->i32{
    for i in 1..=5{
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    return 5;
}
#[tokio::main]
async fn main(){
    let future = r1();

    match tokio::time::timeout(Duration::from_millis(600), future).await {
        Ok(result) => println!("completed: {:?}", result),
        Err(_)     => println!("timed out"),
    }
}