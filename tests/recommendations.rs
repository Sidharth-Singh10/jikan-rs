use common::rate_limited_test;
use jikan_rs::JikanClient;
mod common;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn get_recent_anime_recommendations() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_recent_anime_recommendations(None).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn get_recent_anime_recommendations_with_page() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_recent_anime_recommendations(Some(1)).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn get_recent_manga_recommendations() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_recent_manga_recommendations(None).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn get_recent_manga_recommendations_with_page() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_recent_manga_recommendations(Some(1)).await;
        assert!(result.is_ok());
    })
    .await;
}
