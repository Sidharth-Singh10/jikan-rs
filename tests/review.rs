use common::rate_limited_test;
use jikan_rs::JikanClient;
mod common;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
pub async fn get_recent_anime_reviews() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_recent_anime_reviews(Some(1), None, None).await;

        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
pub async fn get_recent_anime_reviews_with_preliminary() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client
            .get_recent_anime_reviews(Some(1), Some(true), None)
            .await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
pub async fn get_recent_anime_reviews_with_spoilers() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client
            .get_recent_anime_reviews(Some(1), None, Some(true))
            .await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
pub async fn get_recent_manga_reviews() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_recent_manga_reviews(Some(1), None, None).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
pub async fn get_recent_manga_reviews_with_preliminary() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client
            .get_recent_manga_reviews(Some(1), Some(true), None)
            .await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
pub async fn get_recent_manga_reviews_with_spoilers() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client
            .get_recent_manga_reviews(Some(1), None, Some(true))
            .await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
pub async fn get_recent_anime_reviews_with_all_params() {
    rate_limited_test(|| async {
        let client = JikanClient::new();

        // Test anime reviews with all parameters
        let anime_result = client
            .get_recent_anime_reviews(Some(1), Some(true), Some(true))
            .await;
        assert!(anime_result.is_ok());
    })
    .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
pub async fn get_recent_manga_reviews_with_all_params() {
    rate_limited_test(|| async {
        // Test manga reviews with all parameters
        let client = JikanClient::new();
        let manga_result = client
            .get_recent_manga_reviews(Some(1), Some(true), Some(true))
            .await;
        assert!(manga_result.is_ok());
    })
    .await;
}
