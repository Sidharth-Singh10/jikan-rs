use common::rate_limited_test;
use jikan_rs::{JikanClient, schedule::ScheduleFilter};
mod common;

#[tokio::test]
async fn get_schedules() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client
            .get_schedules(None, None, None, None, None, None)
            .await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
async fn get_schedules_with_filter() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client
            .get_schedules(Some(ScheduleFilter::Monday), None, None, None, None, None)
            .await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
async fn get_schedules_with_kids_filter() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client
            .get_schedules(None, Some(true), None, None, None, None)
            .await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
async fn get_schedules_with_sfw_filter() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client
            .get_schedules(None, None, Some(true), None, None, None)
            .await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
async fn get_schedules_with_pagination() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client
            .get_schedules(None, None, None, None, Some(1), Some(5))
            .await;
        assert!(result.is_ok());
        if let Ok(response) = result {
            // Check that pagination is working as expected
            assert!(response.data.len() <= 5);
        }
    }).await;
}

#[tokio::test]
async fn get_schedules_with_multiple_params() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client
            .get_schedules(
                Some(ScheduleFilter::Tuesday),
                Some(false),
                Some(true),
                None,
                Some(1),
                Some(10),
            )
            .await;
        assert!(result.is_ok());
    }).await;
}

#[tokio::test]
async fn get_schedules_with_unapproved() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client
            .get_schedules(None, None, None, Some(true), None, None)
            .await;
        assert!(result.is_ok());
    }).await;
}

#[tokio::test]
async fn get_schedules_data_access() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client
            .get_schedules(None, None, None, None, Some(1), Some(3))
            .await;

        if let Ok(response) = result {
            // Test we can access the data fields
            if !response.data.is_empty() {
                let first_anime = &response.data[0];
                // Just access some fields to ensure the struct is properly deserialized
                let _mal_id = first_anime.mal_id;
                let _title = &first_anime.title;
            }

            // Test pagination
            let _last_page = response.pagination.last_visible_page;
            let _has_next = response.pagination.has_next_page;
        } else {
            assert!(false, "Response was not Ok");
        }
    }).await;
}

#[tokio::test]
async fn get_schedules_with_invalid_page() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        // Using a very high page number that's unlikely to exist
        let result = client
            .get_schedules(None, None, None, None, Some(9999), None)
            .await;

        // This should still be Ok but with empty data
        if let Ok(response) = result {
            assert!(response.data.is_empty());
        } else {
            assert!(false, "Response should be Ok with empty data");
        }
    }).await;
}
