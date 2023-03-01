use tokio::sync::broadcast;
use tokio::time::{sleep, Duration};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = broadcast::channel(10);
    let t = Arc::new(tx);
    let t2 = t.clone();
    let t3 = t.clone();
    // Start the counter task
    tokio::spawn(async move {
        let mut count = 0;
        loop {
            sleep(Duration::from_secs(1)).await;
            count += 1;
            t2.send(count).unwrap();
        }
    });

    // Start the receiver tasks
    for _ in 0..3 {
        let mut rx = t3.subscribe();
        tokio::spawn(async move {
            loop {
                let msg = rx.recv().await.unwrap();
                println!("Received message: {}", msg);
            }
        });
    }

    // Wait for the tasks to finish
    tokio::signal::ctrl_c().await.unwrap();
}
