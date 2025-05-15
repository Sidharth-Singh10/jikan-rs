pub mod common;
use crate::common::macs::NamedTestJob;
use jikan_rs::common::enums::{character::CharacterOrder, common::Sort};

ratelimited_test_runner!(
    run_ratelimited_tests,
    [
        make_client_test!(get_character_by_id, client, client.get_character_by_id(1)),
        make_client_test!(
            get_character_full_by_id,
            client,
            client.get_character_full_by_id(1)
        ),
        make_client_test!(get_character_anime, client, client.get_character_anime(1)),
        make_client_test!(get_character_manga, client, client.get_character_manga(1)),
        make_client_test!(get_character_voices, client, client.get_character_voices(1)),
        make_client_test!(get_characters, client, client.get_characters()),
        make_client_test!(
            get_character_pictures,
            client,
            client.get_character_pictures(1)
        ),
        make_client_test!(
            get_character_search_with_query,
            client,
            client.get_character_search(
                Some(1),
                Some(10),
                Some("Naruto".to_string()),
                Some(CharacterOrder::Favorites),
                Some(Sort::Asc),
                None,
            )
        ),
    ]
);
