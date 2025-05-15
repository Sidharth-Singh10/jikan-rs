mod common;
use crate::common::macs::NamedTestJob;

ratelimited_test_runner!(
    run_ratelimited_tests,
    [
        make_client_test!(
            get_watch_popular_episodes,
            client,
            client.get_watch_popular_episodes()
        ),
        make_client_test!(
            get_watch_recent_episodes,
            client,
            client.get_watch_recent_episodes()
        ),
        make_client_test!(
            get_watch_recent_promos,
            client,
            client.get_watch_recent_promos(None)
        ),
        make_client_test!(
            get_watch_recent_promos_with_page,
            client,
            client.get_watch_recent_promos(Some(1))
        ),
        make_client_test!(
            get_watch_popular_promos,
            client,
            client.get_watch_popular_promos()
        ),
    ]
);
