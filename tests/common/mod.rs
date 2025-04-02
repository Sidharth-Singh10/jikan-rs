use lazy_static::lazy_static;
use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};
use tokio::time::{Duration, Instant, sleep};

lazy_static! {
    static ref SECOND_LIMIT: Arc<Semaphore> = Arc::new(Semaphore::new(3)); // Max 3 per second
    static ref MINUTE_LIMIT: Arc<Semaphore> = Arc::new(Semaphore::new(60)); // Max 60 per minute
    static ref LAST_RUN: Arc<Mutex<Instant>> = Arc::new(Mutex::new(Instant::now()));
}

pub async fn rate_limited_test<F, Fut>(test: F)
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = ()>,
{
    let _second_permit = SECOND_LIMIT.acquire().await.unwrap();
    let _minute_permit = MINUTE_LIMIT.acquire().await.unwrap();

    let now = Instant::now();
    let mut last_run = LAST_RUN.lock().await;

    let elapsed = now.duration_since(*last_run);
    if elapsed < Duration::from_millis(350) {
        sleep(Duration::from_millis(350) - elapsed).await;
    }

    *last_run = Instant::now();

    test().await;

    // **Ensure a buffer before the next test runs**
    sleep(Duration::from_millis(50)).await;
}

// Helper function to handle rate limiting between tests
pub async fn wait_between_tests() {
    sleep(Duration::from_secs(2)).await;
}
