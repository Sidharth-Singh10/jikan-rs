use crate::common::rate_limited_test;
use crate::common::wait_between_tests;
use jikan_rs::JikanClient;
use jikan_rs::genre::GenreFilter;
mod common;

#[tokio::test]
async fn get_anime_genres() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_anime_genres(GenreFilter::None).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
async fn get_anime_genres_genres() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_anime_genres(GenreFilter::Genres).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
async fn get_anime_genres_explicit() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_anime_genres(GenreFilter::ExplicitGenres).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
async fn get_anime_genres_themes() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_anime_genres(GenreFilter::Themes).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
async fn get_anime_genres_demographics() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_anime_genres(GenreFilter::Demographics).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
async fn get_manga_genres() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_manga_genres(GenreFilter::None).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
async fn get_manga_genres_genres() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_manga_genres(GenreFilter::Genres).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
async fn get_manga_genres_explicit() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_manga_genres(GenreFilter::ExplicitGenres).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
async fn get_manga_genres_themes() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_manga_genres(GenreFilter::Themes).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
async fn get_manga_genres_demographics() {
    let client = JikanClient::new();
    let result = client.get_manga_genres(GenreFilter::Demographics).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}
