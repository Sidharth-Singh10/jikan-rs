mod common;
use crate::common::macs::NamedTestJob;

ratelimited_test_runner!(
    run_ratelimited_tests,
    [
        make_client_test!(
            get_recent_anime_recommendations,
            client,
            client.get_recent_anime_recommendations(None)
        ),
        make_client_test!(
            get_recent_anime_recommendations_with_page,
            client,
            client.get_recent_anime_recommendations(Some(1))
        ),
        make_client_test!(
            get_recent_manga_recommendations,
            client,
            client.get_recent_manga_recommendations(None)
        ),
        make_client_test!(
            get_recent_manga_recommendations_with_page,
            client,
            client.get_recent_manga_recommendations(Some(1))
        ),
    ]
);
