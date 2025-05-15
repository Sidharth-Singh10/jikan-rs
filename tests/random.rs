pub mod common;
use crate::common::macs::NamedTestJob;

ratelimited_test_runner!(
    run_ratelimited_tests,
    [
        make_client_test!(get_random_anime, client, client.get_random_anime()),
        make_client_test!(get_random_manga, client, client.get_random_manga()),
        make_client_test!(get_random_user, client, client.get_random_user()),
        make_client_test!(get_random_character, client, client.get_random_character()),
        make_client_test!(get_random_people, client, client.get_random_people()),
    ]
);
