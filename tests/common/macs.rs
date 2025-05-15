use std::sync::Arc;

// Each test has a name and a function to execute
pub struct NamedTestJob {
    pub name: &'static str,
    pub func: Arc<dyn Fn() -> tokio::task::JoinHandle<()> + Send + Sync>,
}

// Define the macro to create test jobs
#[macro_export]
macro_rules! make_client_test {
    ($name:ident, $client_var:ident, $call:expr) => {{
        $crate::common::macs::NamedTestJob {
            name: stringify!($name),
            func: std::sync::Arc::new(|| {
                tokio::spawn(async {
                    use tokio::time::{Duration, timeout};
                    let $client_var = jikan_rs::JikanClient::new();
                    let result = timeout(Duration::from_secs(10), async { $call.await }).await;
                    match result {
                        Ok(inner) => {
                            assert!(
                                inner.is_ok(),
                                "Request returned an error: {:?}",
                                inner.err()
                            );
                        }
                        Err(_) => {
                            panic!("Request timed out after 10 seconds");
                        }
                    }
                })
            }),
        }
    }};
}

// Macro for Error Tests
#[macro_export]
macro_rules! make_error_test {
    ($name:ident, $client_var:ident, $call:expr) => {{
        $crate::common::macs::NamedTestJob {
            name: stringify!($name),
            func: std::sync::Arc::new(|| {
                tokio::spawn(async {
                    use tokio::time::{Duration, timeout};
                    let $client_var = jikan_rs::JikanClient::new();
                    let result = timeout(Duration::from_secs(10), async { $call.await }).await;
                    match result {
                        Ok(inner) => {
                            assert!(matches!(inner, Err(JikanError::NotFound)));
                        }
                        Err(_) => {
                            panic!("Request timed out after 10 seconds");
                        }
                    }
                })
            }),
        }
    }};
}

// Define the macro for running rate-limited tests
#[macro_export]
macro_rules! ratelimited_test_runner {
    ($name:ident, [$($test:expr),* $(,)?]) => {
        #[tokio::test]
        async fn $name() {
            use std::sync::atomic::{AtomicBool, Ordering};
            use std::time::{Duration, Instant};
            use tokio::sync::{mpsc, Mutex, Semaphore};
            use std::sync::Arc;
            use tokio::time::sleep;

            let (tx, rx) = mpsc::channel::<NamedTestJob>(100);
            let rx = Arc::new(Mutex::new(rx));
            let has_failed = Arc::new(AtomicBool::new(false));
            let rate_limiter = Arc::new(Semaphore::new(0)); // starts with no permits

            // Global rate limiter: 1 permit per second
            {
                let rate_limiter = Arc::clone(&rate_limiter);
                tokio::spawn(async move {
                    loop {
                        sleep(Duration::from_secs(1)).await;
                        rate_limiter.add_permits(1);
                    }
                });
            }

            let mut sz = 0u64;

            // Enqueue jobs
            $(
                tx.send($test).await.unwrap();
                sz += 1;
            )*

            println!("Running {} tests...", sz);
            drop(tx); // Close the sender

            let mut handles = Vec::new();

            for _ in 0..3 {
                let rx = Arc::clone(&rx);
                let has_failed = Arc::clone(&has_failed);
                let rate_limiter = Arc::clone(&rate_limiter);

                handles.push(tokio::spawn(async move {
                    loop {
                        if has_failed.load(Ordering::SeqCst) {
                            break;
                        }

                        let job_opt = {
                            let mut locked_rx = rx.lock().await;
                            locked_rx.recv().await
                        };

                        #[allow(non_snake_case)]
                        let job = match job_opt {
                            Some(j) => j,
                            None => break, // All jobs done
                        };

                        // Acquire global rate limiter permit
                        rate_limiter.acquire().await.unwrap().forget();

                        let result = (job.func)().await;

                        match result {
                            Ok(_) => println!("{} ... ok", job.name),
                            Err(_) => println!("{} ... FAILED", job.name),
                        }

                        if result.is_err() {
                            has_failed.store(true, Ordering::SeqCst);
                        }
                    }
                }));
            }

            let start_time = Instant::now();

            for handle in handles {
                handle.await.unwrap();
            }

            assert!(!has_failed.load(Ordering::SeqCst), "Some tests failed");

            let elapsed = start_time.elapsed();
            if elapsed < Duration::from_secs(sz) {
                sleep(Duration::from_secs(sz) - elapsed).await;
            }
        }
    };
}
