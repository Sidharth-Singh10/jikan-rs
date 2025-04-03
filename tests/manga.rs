use crate::common::rate_limited_test;
use jikan_rs::JikanClient;
mod common;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn get_manga() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_manga(1).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn get_manga_full() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_manga_full(1).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn get_manga_characters() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_manga_characters(1).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn get_manga_news() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_manga_news(1, Some(1)).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn get_manga_forum() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_manga_forum(1, None).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn get_manga_pictures() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_manga_pictures(1).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn get_manga_statistics() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_manga_statistics(1).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn get_manga_moreinfo() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_manga_moreinfo(1).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn get_manga_recommendations() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_manga_recommendations(1).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn get_manga_userupdates() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_manga_userupdates(1, Some(1)).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn get_manga_reviews() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_manga_reviews(1, Some(1), None, None).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn get_manga_relations() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_manga_relations(1).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn get_manga_external() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_manga_external(1).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn get_nonexistent_manga() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_manga(999999999).await;
        assert!(result.is_err());
    })
    .await;
}
