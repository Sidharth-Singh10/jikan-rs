use crate::common::wait_between_tests;
use jikan_rs::JikanClient;
use jikan_rs::seasons::FilterType;
use serial_test::serial;
mod common;

#[tokio::test]
#[serial]
pub async fn get_season_now() {
    let client = JikanClient::new();
    let result = client
        .get_season_now(None, None, None, None, None, None)
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
pub async fn get_season_now_with_filters() {
    let client = JikanClient::new();
    let result = client
        .get_season_now(
            Some(FilterType::TV),
            Some(true),
            None,
            None,
            Some(1),
            Some(10),
        )
        .await;
    println!("{:?}", result);
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
pub async fn get_specific_season() {
    let client = JikanClient::new();
    // Testing with a known season
    let result = client
        .get_season(2023, "winter", None, None, None, None, None, None)
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
pub async fn get_specific_season_with_filters() {
    let client = JikanClient::new();
    let result = client
        .get_season(
            2023,
            "winter",
            Some(FilterType::Movie),
            Some(true),
            None,
            None,
            Some(1),
            Some(5),
        )
        .await;
    println!("{:?}", result);
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
pub async fn get_seasons_list() {
    let client = JikanClient::new();
    let result = client.get_seasons_list().await;
    println!("{:?}", result);
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
pub async fn get_season_upcoming() {
    let client = JikanClient::new();
    let result = client
        .get_season_upcoming(None, None, None, None, None, None)
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
pub async fn get_season_upcoming_with_filters() {
    let client = JikanClient::new();
    let result = client
        .get_season_upcoming(
            Some(FilterType::TV),
            Some(true),
            None,
            Some(true),
            Some(1),
            Some(10),
        )
        .await;
    println!("{:?}", result);
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
pub async fn test_season_filter_types() {
    let client = JikanClient::new();

    // Testing different filter types
    let filters = vec![
        FilterType::TV,
        FilterType::Movie,
        FilterType::OVA,
        FilterType::ONA,
        FilterType::Special,
        FilterType::Music,
    ];

    for filter in filters {
        let result = client
            .get_season_now(Some(filter), None, None, None, Some(1), Some(3))
            .await;
        assert!(result.is_ok());
        wait_between_tests().await;
    }
}
