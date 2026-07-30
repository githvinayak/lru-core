use std::time::Duration;
use async_trait::async_trait;
#[async_trait]
trait AsyncWorker{
    async fn process(&self,task:i32);
}

struct PrintWorker;


impl PrintWorker{
    pub fn new()->Self{
        PrintWorker{}
    }
}
#[async_trait]
impl AsyncWorker for PrintWorker{
    async fn process(&self,task:i32){
        tokio::time::sleep(Duration::from_millis(100)).await;
        println!("PrintWorker received {}",task);
    }
}

#[tokio::main]
async fn main() {

    let worker = PrintWorker::new();
    worker.process(5).await;
}