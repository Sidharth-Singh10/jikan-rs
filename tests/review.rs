mod common;
use crate::common::macs::NamedTestJob;

ratelimited_test_runner!(
    run_ratelimited_tests,
    [
        make_client_test!(
            get_recent_anime_reviews,
            client,
            client.get_recent_anime_reviews(None, None, None)
        ),
        make_client_test!(
            get_recent_anime_reviews_with_preliminary,
            client,
            client.get_recent_anime_reviews(Some(1), Some(true), None)
        ),
        make_client_test!(
            get_recent_anime_reviews_with_spoilers,
            client,
            client.get_recent_anime_reviews(Some(1), None, Some(true))
        ),
        make_client_test!(
            get_recent_manga_reviews,
            client,
            client.get_recent_manga_reviews(None, None, None)
        ),
        make_client_test!(
            get_recent_manga_reviews_with_preliminary,
            client,
            client.get_recent_manga_reviews(Some(1), Some(true), None)
        ),
        make_client_test!(
            get_recent_manga_reviews_with_spoilers,
            client,
            client.get_recent_manga_reviews(Some(1), None, Some(true))
        ),
        make_client_test!(
            get_recent_anime_reviews_with_all_params,
            client,
            client.get_recent_anime_reviews(Some(1), Some(true), Some(true))
        ),
        make_client_test!(
            get_recent_manga_reviews_with_all_params,
            client,
            client.get_recent_manga_reviews(Some(1), Some(true), Some(true))
        ),
    ]
);
