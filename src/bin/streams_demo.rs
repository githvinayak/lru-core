use tokio_stream::{self, StreamExt};
use tokio::time::Duration;

#[tokio::main]
async fn main() {

    let mut stream = tokio_stream::iter(vec![1, 2, 3]);
    while let Some(val) = stream.next().await {
        tokio::time::sleep(Duration::from_millis(100)).await;
        println!("{}", val);
    }
}