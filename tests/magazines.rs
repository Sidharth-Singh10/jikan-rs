use crate::common::rate_limited_test;
use crate::common::wait_between_tests;
use jikan_rs::JikanClient;
use jikan_rs::magazines::*;
mod common;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn get_magazines_no_params() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client
            .get_magazines(None, None, None, OrderBy::None, Sort::None, None)
            .await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn get_magazines_with_page() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client
            .get_magazines(Some(1), None, None, OrderBy::None, Sort::None, None)
            .await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn get_magazines_with_limit() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client
            .get_magazines(None, Some(10), None, OrderBy::None, Sort::None, None)
            .await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn get_magazines_with_query() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client
            .get_magazines(
                None,
                None,
                Some("Shonen".to_string()),
                OrderBy::None,
                Sort::None,
                None,
            )
            .await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn get_magazines_with_order() {
    let client = JikanClient::new();
    let result = client
        .get_magazines(None, None, None, OrderBy::Name, Sort::None, None)
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn get_magazines_with_sort() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client
            .get_magazines(None, None, None, OrderBy::None, Sort::Asc, None)
            .await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn get_magazines_with_letter() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client
            .get_magazines(
                None,
                None,
                None,
                OrderBy::None,
                Sort::None,
                Some("A".to_string()),
            )
            .await;
        assert!(result.is_ok());
    })
    .await;
}
