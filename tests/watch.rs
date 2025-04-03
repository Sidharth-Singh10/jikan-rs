use common::rate_limited_test;
use jikan_rs::JikanClient;

mod common;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_get_watch_recent_episodes() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_watch_recent_episodes().await;
        match result {
            Ok(response) => {
                assert!(
                    !response.data.is_empty(),
                    "Recent episodes response should not be empty"
                );
                assert!(
                    response.pagination.last_visible_page >= 1,
                    "Should have a valid page number"
                );
            }
            Err(e) => panic!("Failed to fetch recent episodes: {}", e),
        }
    })
    .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_get_watch_popular_episodes() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_watch_popular_episodes().await;
        match result {
            Ok(response) => {
                assert!(
                    !response.data.is_empty(),
                    "Popular episodes response should not be empty"
                );
                assert!(
                    response.pagination.last_visible_page >= 1,
                    "Should have a valid page number"
                );
            }
            Err(e) => panic!("Failed to fetch popular episodes: {}", e),
        }
    })
    .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_get_watch_recent_promos_no_page() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_watch_recent_promos(None).await;
        match result {
            Ok(response) => {
                assert!(
                    !response.data.is_empty(),
                    "Recent promos response should not be empty"
                );
                assert!(
                    response.pagination.last_visible_page >= 1,
                    "Should have a valid page number"
                );
                assert!(!response.title.is_empty(), "Response should have a title");
            }
            Err(e) => panic!("Failed to fetch recent promos without page: {}", e),
        }
    })
    .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_get_watch_recent_promos_with_page() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_watch_recent_promos(Some(1)).await;
        match result {
            Ok(response) => {
                assert!(
                    !response.data.is_empty(),
                    "Recent promos response should not be empty"
                );
                assert!(
                    response.pagination.last_visible_page >= 1,
                    "Should have a valid page number"
                );
                assert!(!response.title.is_empty(), "Response should have a title");
            }
            Err(e) => panic!("Failed to fetch recent promos with page: {}", e),
        }
    })
    .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_get_watch_popular_promos() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_watch_popular_promos().await;

        match result {
            Ok(response) => {
                assert!(
                    !response.data.is_empty(),
                    "Popular promos response should not be empty"
                );
                assert!(
                    response.pagination.last_visible_page >= 1,
                    "Should have a valid page number"
                );
                assert!(!response.title.is_empty(), "Response should have a title");
            }
            Err(e) => panic!("Failed to fetch popular promos: {}", e),
        }
    })
    .await;
}
