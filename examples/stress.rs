use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio;
use tokio::time::{Duration, Instant};

#[tokio::main]
async fn main() {
    let counters: Vec<_> = (0..16).map(|_| Arc::new(AtomicU64::new(0))).collect();

    counters
        .iter()
        .cloned()
        .map(|counter| tokio::spawn(stress_task(counter)))
        .for_each(drop);

    let mut last_sum = 0;
    let mut last_time = Instant::now();
    loop {
        tokio::time::delay_for(Duration::from_secs(1)).await;

        let new_sum = total_count(&counters);
        let new_time = Instant::now();
        println!(
            "throughput = {:?}",
            (new_sum - last_sum) as f32 / (new_time - last_time).as_nanos() as f32 * 1.0e9
        );
        last_sum = new_sum;
        last_time = new_time;
    }
}

fn total_count(counters: &[Arc<AtomicU64>]) -> u64 {
    counters
        .iter()
        .map(|counter| counter.load(Ordering::Relaxed))
        .sum()
}

async fn stress_task(counter: Arc<AtomicU64>) {
    let client = reqwest::Client::new();

    loop {
        //        println!("Sending request");
        let res = client
            .get("http://localhost:8080/reverse/abcdefgh")
            .send()
            .await
            .unwrap();
        counter.fetch_add(1, Ordering::Relaxed);

        let result = res.text().await.unwrap();
        if &result != "hgfedcba" {
            println!("Got wrong result {}", result);
        }
        //        tokio::time::delay_for(Duration::from_millis(2000)).await;
    }
}
