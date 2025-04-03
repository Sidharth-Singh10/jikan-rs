use common::rate_limited_test;
use jikan_rs::JikanClient;
mod common;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn get_anime_random() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_random_anime().await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn get_manga_random() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_random_manga().await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn get_user_random() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_random_user().await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn get_character_random() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_random_character().await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn get_person_random() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_random_person().await;
        assert!(result.is_ok());
    })
    .await;
}
