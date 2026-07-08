use std::time::Duration;
use std::time::Instant;
// async fn add(a: i32, b: i32) -> i32 {
//     a + b
// }

async fn r1(){
    for i in 1..=1000{
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    println!("done r`");
}
async fn r2(){
    for i in 1..=1000{
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    println!("done r2");
}

#[tokio::main]
async fn main() {
    // let future = add(2, 3).await;
    // println!("future created {:?}", future);

    // let mut handles = vec![];
    // for i in 1..4{
    //     let task = tokio::spawn(async move {
    //         tokio::time::sleep(Duration::from_millis(1000)).await;
    //         println!("{}", i);
    //     });
    //     handles.push(task);
    // }
    //
    // for handle in handles{
    //     handle.await.unwrap();
    // }

    let start = Instant::now();
    let r1_future = r1().await;
    let r2_future = r2().await;
     println!("took `{:?}`", start.elapsed());

    let start = Instant::now();
    let (r1,r2) = tokio::join!(r1(),r2());
    println!("took `{:?}`", start.elapsed());

}