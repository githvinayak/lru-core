async fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let future = add(2, 3);  // what happens here?
    println!("future created");
}