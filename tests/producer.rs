mod common;
use crate::common::macs::NamedTestJob;

ratelimited_test_runner!(
    run_ratelimited_tests,
    [
        make_client_test!(get_producer_by_id, client, client.get_producer_by_id(1)),
        make_client_test!(get_producer_full, client, client.get_producer_full_by_id(1)),
        make_client_test!(
            get_producer_external,
            client,
            client.get_producer_external(1)
        ),
        make_client_test!(
            get_producer_search,
            client,
            client.get_producer_search(None, None, None, None, None, Some(String::from("m")))
        ),
        make_client_test!(get_producers, client, client.get_producers()),
    ]
);
