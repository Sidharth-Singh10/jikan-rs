mod common;
use crate::common::macs::NamedTestJob;

ratelimited_test_runner!(
    run_ratelimited_tests,
    [
        make_client_test!(get_person_by_id, client, client.get_person_by_id(1)),
        make_client_test!(get_person_full, client, client.get_person_by_id_full(1)),
        make_client_test!(get_person_anime, client, client.get_person_anime(1)),
        make_client_test!(get_person_manga, client, client.get_person_manga(2619)), // Araki, Hirohiko
        make_client_test!(get_person_voice, client, client.get_person_voice(195)), // Junichi Suwabe(Ryomen Sukuna)
        make_client_test!(get_person_pictures, client, client.get_person_pictures(1)),
        make_client_test!(get_people, client, client.get_people()),
        make_client_test!(
            get_people_search,
            client,
            client.get_people_search(
                None,
                None,
                Some("Junichi Suwabe".to_string()),
                None,
                None,
                None
            )
        ),
    ]
);
