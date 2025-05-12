use crate::common::wait_between_tests;
use jikan_rs::JikanClient;
use serial_test::serial;
mod common;

#[tokio::test]
#[serial]
pub async fn get_recent_anime_reviews() {
    let client = JikanClient::new();
    let result = client.get_recent_anime_reviews(Some(1), None, None).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
pub async fn get_recent_anime_reviews_with_preliminary() {
    let client = JikanClient::new();
    let result = client
        .get_recent_anime_reviews(Some(1), Some(true), None)
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
pub async fn get_recent_anime_reviews_with_spoilers() {
    let client = JikanClient::new();
    let result = client
        .get_recent_anime_reviews(Some(1), None, Some(true))
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
pub async fn get_recent_manga_reviews() {
    let client = JikanClient::new();
    let result = client.get_recent_manga_reviews(Some(1), None, None).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
pub async fn get_recent_manga_reviews_with_preliminary() {
    let client = JikanClient::new();
    let result = client
        .get_recent_manga_reviews(Some(1), Some(true), None)
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
pub async fn get_recent_manga_reviews_with_spoilers() {
    let client = JikanClient::new();
    let result = client
        .get_recent_manga_reviews(Some(1), None, Some(true))
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
pub async fn get_recent_anime_reviews_with_all_params() {
    let client = JikanClient::new();
    let result = client
        .get_recent_anime_reviews(Some(1), Some(true), Some(true))
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
pub async fn get_recent_manga_reviews_with_all_params() {
    let client = JikanClient::new();
    let result = client
        .get_recent_manga_reviews(Some(1), Some(true), Some(true))
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}
