async fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[tokio::main]
async fn main() {
    let future = add(2, 3).await;
    println!("future created {:?}", future);
}