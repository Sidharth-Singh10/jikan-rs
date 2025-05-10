use crate::common::wait_between_tests;
use jikan_rs::{
    JikanClient, JikanError,
    anime::{AnimeType, OrderBy, Rating, SearchParams, Sort, Status},
};
use serial_test::serial;
mod common;

#[tokio::test]
#[serial]
async fn get_anime() {
    let client = JikanClient::new();
    let result = client.get_anime(1).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_anime_search() {
    let client = JikanClient::new();
    let params = SearchParams {
        q: Some("Death"),
        status: Some(Status::Complete),
        sfw: Some(false),
        limit: Some(10),
        type_: Some(AnimeType::TV),
        unapproved: Some(false),
        page: Some(1),
        // score: Some(8.62),   //* score can not be provided if there is an min_score or max_score
        min_score: Some(2.00), //* min_score and max_score will be ignored if score is passed
        max_score: Some(9.00),
        rating: Some(Rating::Pg13),
        genres: Some("10"),
        genres_exclude: Some("2"),
        order_by: Some(OrderBy::MalId),
        sort: Some(Sort::Asc),
        // letter: Some("d"),     //* this param can not be provided alongside the q param
        producers: Some("102"),
        start_date: Some("1997-01-01"), //* the param for 'start_date' and 'end_date' MUST follow the YYYY-MM-DD date format
        end_date: Some("2025-05-01"),
        ..Default::default()
    };
    let result = client.get_anime_search(Some(params)).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_anime_full() {
    let client = JikanClient::new();
    let result = client.get_anime_full(1).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_anime_characters() {
    let client = JikanClient::new();
    let result = client.get_anime_characters(1).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_anime_staff() {
    let client = JikanClient::new();
    let result = client.get_anime_staff(1).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_anime_episodes() {
    let client = JikanClient::new();
    let result = client.get_anime_episodes(1, Some(1)).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_anime_videos() {
    let client = JikanClient::new();
    let result = client.get_anime_videos(1).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_anime_news() {
    let client = JikanClient::new();
    let result = client.get_anime_news(1, Some(1)).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_anime_forum() {
    let client = JikanClient::new();
    let result = client.get_anime_forum(1, None).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_anime_themes() {
    let client = JikanClient::new();
    let result = client.get_anime_themes(1).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_anime_recommendations() {
    let client = JikanClient::new();
    let result = client.get_anime_recommendations(1).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_anime_userupdates() {
    let client = JikanClient::new();
    let result = client.get_anime_userupdates(1, Some(1)).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_anime_reviews() {
    let client = JikanClient::new();
    let result = client.get_anime_reviews(1, Some(1), None, None).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_anime_external() {
    let client = JikanClient::new();
    let result = client.get_anime_external(1).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_anime_streaming() {
    let client = JikanClient::new();
    let result = client.get_anime_streaming(1).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_nonexistent_anime() {
    let client = JikanClient::new();
    let result = client.get_anime(999999999).await;
    assert!(matches!(result, Err(JikanError::NotFound)));
    wait_between_tests().await;
}
