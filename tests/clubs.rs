use crate::common::rate_limited_test;
use jikan_rs::{JikanClient, JikanError, clubs::ClubSearchParams};
mod common;

#[tokio::test]
async fn get_club_by_id() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_club_by_id(1).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
async fn get_club_members() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_club_members(1, Some(1)).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
async fn get_club_staff() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_club_staff(1).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
async fn get_club_relations() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_club_relations(1).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
async fn get_club_search() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let params = ClubSearchParams::new()
            .with_page(1)
            .with_limit(10)
            .with_query("anime");

        let result = client.get_club_search(params).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
async fn get_club_search_with_category() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let params = ClubSearchParams::new()
            .with_page(1)
            .with_limit(5)
            .with_category("manga");

        let result = client.get_club_search(params).await;
        assert!(result.is_ok());
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
async fn get_club_search_with_order() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let params = ClubSearchParams::new()
            .with_page(1)
            .with_limit(10)
            .with_order_by("members_count")
            .with_sort("desc");

        let result = client.get_club_search(params).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
async fn get_club_search_with_letter() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let params = ClubSearchParams::new()
            .with_page(1)
            .with_limit(10)
            .with_letter("A");

        let result = client.get_club_search(params).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
async fn get_nonexistent_club() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_club_by_id(999999999).await;
        assert!(matches!(result, Err(JikanError::NotFound)));
    })
    .await;
}

#[tokio::test]
async fn get_nonexistent_club_members() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_club_members(999999999, Some(1)).await;
        assert!(result.is_err());
    })
    .await;
}

#[tokio::test]
async fn get_nonexistent_club_staff() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_club_staff(999999999).await;
        assert!(result.is_err());
    })
    .await;
}
