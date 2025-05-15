mod common;
use crate::common::macs::NamedTestJob;
use jikan_rs::users::GetUsersParams;

ratelimited_test_runner!(
    run_ratelimited_tests,
    [
        make_client_test!(
            get_user_full,
            client,
            client.get_user_full_profile("Thunder-Blaze")
        ),
        make_client_test!(get_user, client, client.get_user_profile("InSaiyan__")),
        make_client_test!(
            get_users,
            client,
            client.get_user_search(GetUsersParams {
                page: None,
                limit: None,
                q: None,
                gender: None,
                location: None,
                min_age: None,
                max_age: None,
            })
        ),
        make_client_test!(get_user_by_id, client, client.get_user_by_id(15847568)),
        make_client_test!(
            get_user_stats,
            client,
            client.get_user_statistics("TheLlama")
        ),
        make_client_test!(
            get_user_friends,
            client,
            client.get_user_friends("Kisaragi_Toka", Some(1))
        ),
        make_client_test!(
            get_user_reviews,
            client,
            client.get_user_reviews("TheLlama", Some(1))
        ),
        make_client_test!(
            get_user_history,
            client,
            client.get_user_history("Kisaragi_Toka", None)
        ),
        make_client_test!(
            get_user_favorites,
            client,
            client.get_user_favorites("Kisaragi_Toka")
        ),
        make_client_test!(get_user_about, client, client.get_user_about("eren")),
        make_client_test!(
            get_user_external,
            client,
            client.get_user_external("Kisaragi_Toka")
        ),
        make_client_test!(
            get_user_updates,
            client,
            client.get_user_updates("Kisaragi_Toka")
        ),
        make_client_test!(
            get_user_recommendations,
            client,
            client.get_user_recommendations("TheLlama", Some(1))
        ),
        make_client_test!(
            get_user_clubs,
            client,
            client.get_user_clubs("Kisaragi_Toka", Some(1))
        ),
    ]
);
