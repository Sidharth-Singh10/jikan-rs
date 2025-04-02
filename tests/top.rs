use jikan_rs::{JikanClient, top::*};
use common::rate_limited_test;
mod common;

#[tokio::test]
async fn get_top_anime() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client
            .get_top_anime(
                AnimeType::None,
                Filter::None,
                Rating::None,
                None,
                None,
                None,
            )
            .await;
        assert!(result.is_ok());
    }).await;
}

#[tokio::test]
async fn get_top_anime_with_type() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client
            .get_top_anime(AnimeType::Tv, Filter::None, Rating::None, None, None, None)
            .await;
        assert!(result.is_ok());
    }).await;
}

#[tokio::test]
async fn get_top_anime_with_filter() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client
            .get_top_anime(
                AnimeType::None,
                Filter::Airing,
                Rating::None,
                None,
                None,
                None,
            )
            .await;
        assert!(result.is_ok());
    }).await;
}

#[tokio::test]
async fn get_top_anime_with_rating() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client
            .get_top_anime(
                AnimeType::None,
                Filter::None,
                Rating::Pg13,
                None,
                None,
                None,
            )
            .await;
        assert!(result.is_ok());
    }).await;
}

#[tokio::test]
async fn get_top_anime_with_sfw() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client
            .get_top_anime(
                AnimeType::None,
                Filter::None,
                Rating::None,
                Some(true),
                None,
                None,
            )
            .await;
        assert!(result.is_ok());
    }).await;
}

#[tokio::test]
async fn get_top_anime_with_page() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client
            .get_top_anime(
                AnimeType::None,
                Filter::None,
                Rating::None,
                None,
                Some(1),
                None,
            )
            .await;
        assert!(result.is_ok());
    }).await;
}

#[tokio::test]
async fn get_top_anime_with_limit() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client
            .get_top_anime(
                AnimeType::None,
                Filter::None,
                Rating::None,
                None,
                None,
                Some(10),
            )
            .await;
        assert!(result.is_ok());
    }).await;
}

#[tokio::test]
async fn get_top_manga() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client
            .get_top_manga(MangaType::None, MangaFilter::None, None, None)
            .await;
        assert!(result.is_ok());
    }).await;
}

#[tokio::test]
async fn get_top_manga_with_type() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client
            .get_top_manga(MangaType::Manga, MangaFilter::None, None, None)
            .await;
        assert!(result.is_ok());
    }).await;
}

#[tokio::test]
async fn get_top_manga_with_filter() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client
            .get_top_manga(
                MangaType::None,
                MangaFilter::Publishing,
                None,
                None,
            )
            .await;
        assert!(result.is_ok());
    }).await;
}

#[tokio::test]
async fn get_top_manga_with_page() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client
            .get_top_manga(MangaType::None, MangaFilter::None, Some(1), None)
            .await;
        assert!(result.is_ok());
    }).await;
}

#[tokio::test]
async fn get_top_manga_with_limit() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client
            .get_top_manga(MangaType::None, MangaFilter::None, None, Some(10))
            .await;
        assert!(result.is_ok());
    }).await;
}

#[tokio::test]
async fn get_top_people() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_top_people(None, None).await;
        assert!(result.is_ok());
    }).await;
}

#[tokio::test]
async fn get_top_people_with_page() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_top_people(Some(1), None).await;
        assert!(result.is_ok());
    }).await;
}

#[tokio::test]
async fn get_top_people_with_limit() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_top_people(None, Some(10)).await;
        assert!(result.is_ok());
    }).await;
}

#[tokio::test]
async fn get_top_reviews() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client
            .get_top_reviews(ReviewType::None, None, None, None)
            .await;
        assert!(result.is_ok());
    }).await;
}

#[tokio::test]
async fn get_top_reviews_with_type() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client
            .get_top_reviews(ReviewType::Anime, None, None, None)
            .await;
        assert!(result.is_ok());
    }).await;
}

#[tokio::test]
async fn get_top_reviews_with_preliminary() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client
            .get_top_reviews(ReviewType::None, Some(true), None, None)
            .await;
        assert!(result.is_ok());
    }).await;
}

#[tokio::test]
async fn get_top_reviews_with_spoilers() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client
            .get_top_reviews(ReviewType::None, None, Some(true), None)
            .await;
        assert!(result.is_ok());
    }).await;
}

#[tokio::test]
async fn get_top_reviews_with_page() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client
            .get_top_reviews(ReviewType::None, None, None, Some(2))
            .await;
        assert!(result.is_ok());
    }).await;
}
