use lazy_static::lazy_static;
use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};
use tokio::task;
use tokio::time;
use tokio::time::{Duration, Instant, sleep};

lazy_static! {
    static ref SECOND_LIMIT: Arc<Semaphore> = Arc::new(Semaphore::new(3)); // 3 requests per second
    static ref MINUTE_LIMIT: Arc<Semaphore> = Arc::new(Semaphore::new(60)); // 60 requests per minute
    static ref LAST_RUN: Arc<Mutex<Instant>> = Arc::new(Mutex::new(Instant::now()));
}

// Background task to refill the semaphores at correct intervals
pub fn start_rate_limiter() {
    let second_limit = Arc::clone(&SECOND_LIMIT);
    let minute_limit = Arc::clone(&MINUTE_LIMIT);

    task::spawn(async move {
        let mut second_timer = time::interval(Duration::from_secs_f32(1.0 / 3.0)); // Every ~333ms
        let mut minute_timer = time::interval(Duration::from_secs(1));

        loop {
            second_timer.tick().await;
            second_limit.add_permits(1); // Refill 1 request every ~333ms

            minute_timer.tick().await;
            minute_limit.add_permits(1); // Refill 1 request every second
        }
    });
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

    if elapsed < Duration::from_millis(333) {
        sleep(Duration::from_millis(333) - elapsed).await;
    }

    *last_run = Instant::now();
    test().await;
}

// Helper function to handle rate limiting between tests
pub async fn wait_between_tests() {
    sleep(Duration::from_secs(2)).await;
}
