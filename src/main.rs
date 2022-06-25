use futures::future::join_all;
use std::time::Duration;

async fn f1(a: i32) -> i32 {
    println!("f1 start / {}", a);
    tokio::time::sleep(Duration::from_secs(3)).await;
    println!("f1 finish / {}", a);
    a + 1
}

#[tokio::main]
async fn main() {
    let batch = vec![f1(1), f1(2), f1(3)];
    let res = join_all(batch).await;
    dbg!(res);
}
