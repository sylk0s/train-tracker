use train_tracker::run;

#[tokio::main]
async fn main() {
    if let Err(some) = run().await {
        println!("Code error {}", some);
    }
}
