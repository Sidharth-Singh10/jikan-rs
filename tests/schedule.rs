mod common;
use crate::common::macs::NamedTestJob;
use jikan_rs::common::enums::schedule::ScheduleFilter;

ratelimited_test_runner!(
    run_ratelimited_tests,
    [
        make_client_test!(
            get_schedules,
            client,
            client.get_schedules(None, None, None, None, None, None)
        ),
        make_client_test!(
            get_schedules_with_filter,
            client,
            client.get_schedules(Some(ScheduleFilter::Monday), None, None, None, None, None)
        ),
        make_client_test!(
            get_schedules_with_kids_filter,
            client,
            client.get_schedules(None, Some(true), None, None, None, None)
        ),
        make_client_test!(
            get_schedules_with_sfw_filter,
            client,
            client.get_schedules(None, None, Some(true), None, None, None)
        ),
        make_client_test!(
            get_schedules_with_pagination,
            client,
            client.get_schedules(None, None, None, None, Some(1), Some(5))
        ),
        make_client_test!(
            get_schedules_with_multiple_params,
            client,
            client.get_schedules(
                Some(ScheduleFilter::Tuesday),
                Some(false),
                Some(true),
                None,
                Some(1),
                Some(10)
            )
        ),
        make_client_test!(
            get_schedules_with_unapproved,
            client,
            client.get_schedules(None, None, None, Some(true), None, None)
        ),
        make_client_test!(
            get_schedules_data_access,
            client,
            client.get_schedules(None, None, None, None, Some(1), Some(3))
        ),
    ]
);
