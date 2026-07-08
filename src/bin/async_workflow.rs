use std::time::{Duration, Instant};

async fn r1()->i32{
    let a = 1;
    tokio::time::sleep(Duration::from_millis(100)).await;
    return a;
}


async fn r2()->i32{
    let a = 2;
    tokio::time::sleep(Duration::from_millis(100)).await;
    return a;
}
async fn r3()->i32{
    let a = 3;
    tokio::time::sleep(Duration::from_millis(100)).await;
    return a;
}


#[tokio::main]
async fn main(){
  let start  = Instant::now();
    let (r1,r2,r3) = tokio::join!(r1(),r2(),r3());
    println!("took {:?}", start.elapsed());
    println!("{:?}",r1);
    println!("{:?}",r2);
    println!("{:?}",r3);
}