use jikan_rs::JikanClient;
use common::rate_limited_test;
mod common;

#[tokio::test]
async fn get_user_full() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_user_full("InSaiyan__").await; // github.com/In-Saiyan
        assert!(result.is_ok());
    }).await;
}

#[tokio::test]
async fn get_user() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_user("InSaiyan__").await; // github.com/In-Saiyan
        assert!(result.is_ok());
    }).await;
}

#[tokio::test]
async fn get_users() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_users().await;
        assert!(result.is_ok());
    }).await;
}

#[tokio::test]
async fn get_user_by_id() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_user_by_id(15847568).await;
        assert!(result.is_ok());
    }).await;
}

#[tokio::test]
async fn get_user_stats() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_user_stats("InSaiyan__").await; 
        assert!(result.is_ok());
    }).await;
}

#[tokio::test]
async fn get_user_friends() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_user_friends("InSaiyan__").await;
        assert!(result.is_ok());
    }).await;
}

#[tokio::test]
async fn get_user_reviews() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_user_reviews("InSaiyan__").await;
        assert!(result.is_ok());
    }).await;
}

#[tokio::test]
async fn get_user_history() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_user_history("InSaiyan__").await;
        assert!(result.is_ok());
    }).await;
}

#[tokio::test]
async fn get_user_favorites() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_user_favorites("InSaiyan__").await;
        assert!(result.is_ok());
    }).await;
    
}
