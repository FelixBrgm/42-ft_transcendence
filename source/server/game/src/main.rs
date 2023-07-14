mod queue;
use self::queue::queue;

#[tokio::main]
async fn main() {
    // println!("Queue started...");
    queue().await;
}
