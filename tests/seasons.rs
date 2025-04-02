use jikan_rs::JikanClient;
use jikan_rs::seasons::{FilterType, SeasonQueryParams};
use common::rate_limited_test;
mod common;

#[tokio::test]
async fn test_get_season_now() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_season_now(None).await;
        assert!(result.is_ok());
    }).await;
}

#[tokio::test]
async fn test_get_season_now_with_filters() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let params = SeasonQueryParams::new()
            .filter(FilterType::TV)
            .sfw(true)
            .page(1)
            .limit(10);
        let result = client.get_season_now(Some(params)).await;
        assert!(result.is_ok());
    }).await;
}

#[tokio::test]
async fn test_get_specific_season() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_season(2023, "winter", None).await;
        assert!(result.is_ok());
    }).await;
}

#[tokio::test]
async fn test_get_specific_season_with_filters() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let params = SeasonQueryParams::new()
            .filter(FilterType::Movie)
            .sfw(true)
            .page(1)
            .limit(5);
        let result = client.get_season(2023, "winter", Some(params)).await;
        assert!(result.is_ok());
    }).await;
}

#[tokio::test]
async fn test_get_seasons_list() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_seasons_list().await;
        assert!(result.is_ok());
    }).await;
}

#[tokio::test]
async fn test_get_season_upcoming() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_season_upcoming(None).await;
        assert!(result.is_ok());
    }).await;
}

#[tokio::test]
async fn test_get_season_upcoming_with_filters() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let params = SeasonQueryParams::new()
            .filter(FilterType::TV)
            .sfw(true)
            .continuing(true)
            .page(1)
            .limit(10);
        let result = client.get_season_upcoming(Some(params)).await;
        assert!(result.is_ok());
    }).await;
}
