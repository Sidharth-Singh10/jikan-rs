mod common;
use crate::common::macs::NamedTestJob;
use jikan_rs::{common::enums::season::SeasonFilter, seasons::SeasonQueryParams};

ratelimited_test_runner!(
    run_ratelimited_tests,
    [
        make_client_test!(get_season_now, client, client.get_season_now(None)),
        make_client_test!(
            get_season_now_with_filters,
            client,
            client.get_season_now(Some(SeasonQueryParams {
                filter: Some(SeasonFilter::TV),
                sfw: Some(true),
                page: Some(1),
                limit: Some(10),
                ..Default::default()
            }))
        ),
        make_client_test!(
            get_specific_season,
            client,
            client.get_season(2023, "winter", None)
        ),
        make_client_test!(
            get_specific_season_with_filters,
            client,
            client.get_season(
                2023,
                "winter",
                Some(SeasonQueryParams {
                    filter: Some(SeasonFilter::Movie),
                    sfw: Some(true),
                    page: Some(1),
                    limit: Some(5),
                    ..Default::default()
                })
            )
        ),
        make_client_test!(get_seasons_list, client, client.get_seasons_list()),
        make_client_test!(
            get_season_upcoming,
            client,
            client.get_season_upcoming(None)
        ),
        make_client_test!(
            get_season_upcoming_with_filters,
            client,
            client.get_season_upcoming(Some(SeasonQueryParams {
                filter: Some(SeasonFilter::TV),
                sfw: Some(true),
                continuing: Some(true),
                page: Some(1),
                limit: Some(5),
                ..Default::default()
            }))
        ),
    ]
);
