use std::time::Duration;

async fn r1(){
        tokio::time::sleep(Duration::from_millis(100)).await;
    println!("done r1");
}
async fn r2(){
        tokio::time::sleep(Duration::from_millis(200)).await;
    println!("done r2");
}

#[tokio::main]
async fn main(){
    let future_a = r1();
    let future_b = r2();
    tokio::select! {
    result = future_a => {
            println!("future_a task done");
        }
    result = future_b => {
            println!("future_b task done");
        }
};
}