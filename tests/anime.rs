use jikan_rs::{
    anime::SearchParams,
    common::enums::anime::{AnimeOrder, AnimeRating, AnimeStatus, AnimeType},
    common::enums::common::Sort,
};
mod common;
use crate::common::macs::NamedTestJob;

ratelimited_test_runner!(
    run_ratelimited_tests,
    [
        make_client_test!(get_anime, client, client.get_anime(1)),
        make_client_test!(
            get_anime_search,
            client,
            client.get_anime_search(Some(SearchParams {
                q: Some("Death".to_string()),
                status: Some(AnimeStatus::Complete),
                limit: Some(10),
                type_: Some(AnimeType::Movie),
                order_by: Some(AnimeOrder::Title),
                sort: Some(Sort::Asc),
                rating: Some(AnimeRating::R),
                ..Default::default()
            }))
        ),
        make_client_test!(get_anime_full, client, client.get_anime_full(52991)),
        make_client_test!(get_anime_characters, client, client.get_anime_characters(1)),
        make_client_test!(get_anime_staff, client, client.get_anime_staff(1)),
        make_client_test!(
            get_anime_episodes,
            client,
            client.get_anime_episodes(1, None)
        ),
        make_client_test!(get_anime_videos, client, client.get_anime_videos(1)),
        make_client_test!(get_anime_news, client, client.get_anime_news(1, None)),
        make_client_test!(get_anime_forum, client, client.get_anime_forum(1, None)),
        make_client_test!(get_anime_themes, client, client.get_anime_themes(1)),
        make_client_test!(
            get_anime_recommendations,
            client,
            client.get_anime_recommendations(1)
        ),
        make_client_test!(
            get_anime_userupdates,
            client,
            client.get_anime_userupdates(1, None)
        ),
        make_client_test!(
            get_anime_reviews,
            client,
            client.get_anime_reviews(1, None, None, None)
        ),
        make_client_test!(
            get_anime_relations,
            client,
            client.get_anime_relations(51818)
        ),
        make_client_test!(get_anime_external, client, client.get_anime_external(1)),
        make_client_test!(get_anime_streaming, client, client.get_anime_streaming(1)),
    ]
);
