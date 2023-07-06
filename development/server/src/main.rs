mod queue;
use self::queue::queue;

#[tokio::main]
async fn main() {
    queue().await;
}
