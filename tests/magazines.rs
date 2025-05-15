use jikan_rs::common::enums::{common::Sort, magazines::MagazineOrder};
mod common;
use crate::common::macs::NamedTestJob;

ratelimited_test_runner!(
    run_ratelimited_tests,
    [
        make_client_test!(
            get_magazines,
            client,
            client.get_magazines(None, None, None, None, None, None)
        ),
        make_client_test!(
            get_magazines_with_page,
            client,
            client.get_magazines(Some(1), None, None, None, None, None)
        ),
        make_client_test!(
            get_magazines_with_limit,
            client,
            client.get_magazines(None, Some(10), None, None, None, None)
        ),
        make_client_test!(
            get_magazines_with_query,
            client,
            client.get_magazines(None, None, Some("Shonen".to_string()), None, None, None)
        ),
        make_client_test!(
            get_magazines_with_order,
            client,
            client.get_magazines(None, None, None, Some(MagazineOrder::Name), None, None)
        ),
        make_client_test!(
            get_magazines_with_sort,
            client,
            client.get_magazines(
                None,
                None,
                None,
                Some(MagazineOrder::MalId),
                Some(Sort::Desc),
                None,
            )
        ),
        make_client_test!(
            get_magazines_with_letter,
            client,
            client.get_magazines(None, None, None, None, None, Some("A".to_string()))
        ),
    ]
);
