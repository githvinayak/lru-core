use std::time::Duration;
// async fn add(a: i32, b: i32) -> i32 {
//     a + b
// }

#[tokio::main]
async fn main() {
    // let future = add(2, 3).await;
    // println!("future created {:?}", future);

    let mut handles = vec![];
    for i in 1..4{
        let task = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(1000)).await;
            println!("{}", i);
        });
        handles.push(task);
    }

    for handle in handles{
        handle.await.unwrap();
    }

}