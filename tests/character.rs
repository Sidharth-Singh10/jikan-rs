use crate::common::rate_limited_test;
use jikan_rs::JikanClient;
use jikan_rs::character::OrderBy;
use jikan_rs::character::Sort;
mod common;

#[tokio::test]
pub async fn get_character_by_id() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_character_by_id(1).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
pub async fn get_character_full_by_id() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_character_full_by_id(1).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
pub async fn get_character_anime() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_character_anime(1).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
pub async fn get_character_manga() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_character_manga(1).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
pub async fn get_character_voices() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_character_voices(1).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
pub async fn get_characters() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_characters().await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
pub async fn get_character_pictures() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_character_pictures(1).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
pub async fn get_character_search() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client
            .get_character_search(
                None,
                Some(1),
                Some(String::from("Naruto")),
                Some(OrderBy::Favorites),
                Some(Sort::Asc),
                None,
            )
            .await;
        assert!(result.is_ok());
    })
    .await;
}
