use crate::common::rate_limited_test;
use jikan_rs::JikanClient;
mod common;

#[tokio::test]
async fn get_anime() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_anime(1).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
async fn get_anime_full() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_anime_full(1).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
async fn get_anime_characters() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_anime_characters(1).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
async fn get_anime_staff() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_anime_staff(1).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
async fn get_anime_episodes() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_anime_episodes(1, Some(1)).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
async fn get_anime_videos() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_anime_videos(1).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
async fn get_anime_news() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_anime_news(1, Some(1)).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
async fn get_anime_forum() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_anime_forum(1, None).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
async fn get_anime_themes() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_anime_themes(1).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
async fn get_anime_recommendations() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_anime_recommendations(1).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
async fn get_anime_userupdates() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_anime_userupdates(1, Some(1)).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
async fn get_anime_reviews() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_anime_reviews(1, Some(1), None, None).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
async fn get_anime_external() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_anime_external(1).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
async fn get_anime_streaming() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_anime_streaming(1).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
async fn get_nonexistent_anime() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_anime(999999999).await;
        assert!(result.is_err());
    })
    .await;
}
