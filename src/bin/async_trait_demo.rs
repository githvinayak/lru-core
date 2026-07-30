use std::time::Duration;
use async_trait::async_trait;

#[async_trait]
trait AsyncWorker{
    async fn process(&self,task:i32);
}
#[derive(Debug)]
struct PrintWorker;
#[derive(Debug)]
struct LogWorker;

impl PrintWorker{
    pub fn new()->Self{
        PrintWorker{}
    }
}

impl LogWorker{
    pub fn new()->Self{
        LogWorker{}
    }
}


#[async_trait]
impl AsyncWorker for PrintWorker{
    async fn process(&self,task:i32){
        tokio::time::sleep(Duration::from_millis(100)).await;
        println!("PrintWorker received {}",task);
    }
}

#[async_trait]
impl AsyncWorker for LogWorker{
    async fn process(&self,task:i32){
        println!("logged: {task}");
    }
}
#[tokio::main]
async fn main() {
    let workers : Vec<Box<dyn AsyncWorker>> = vec![Box::new(PrintWorker::new()),Box::new(LogWorker::new())];
    for worker in workers{
        worker.process(5).await;
    }
}