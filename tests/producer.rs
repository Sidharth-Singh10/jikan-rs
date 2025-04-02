use common::rate_limited_test;
use jikan_rs::JikanClient;
mod common;

#[tokio::test]
async fn get_producer_by_id() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_producer_by_id(1).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
async fn get_producer_by_id_full() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_producer_full_by_id(1).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
async fn get_producer_external() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_producer_external(1).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
async fn get_producer_search() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client
            .get_producer_search(None, None, None, None, None, Some(String::from("m")))
            .await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
async fn get_producers() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_producers().await;
        assert!(result.is_ok());
    })
    .await;
}
