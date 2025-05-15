use jikan_rs::common::enums::{
    anime::{AnimeFilter, AnimeRating, AnimeType},
    manga::{MangaFilter, MangaType},
    reviews::ReviewType,
};
mod common;
use crate::common::macs::NamedTestJob;

ratelimited_test_runner!(
    run_ratelimited_tests,
    [
        make_client_test!(
            get_top_anime,
            client,
            client.get_top_anime(None, None, None, None, None, None)
        ),
        make_client_test!(
            get_top_anime_with_type,
            client,
            client.get_top_anime(Some(AnimeType::TV), None, None, None, None, None)
        ),
        make_client_test!(
            get_top_anime_with_filter,
            client,
            client.get_top_anime(None, Some(AnimeFilter::Airing), None, None, None, None)
        ),
        make_client_test!(
            get_top_anime_with_rating,
            client,
            client.get_top_anime(None, None, Some(AnimeRating::PG13), None, None, None)
        ),
        make_client_test!(
            get_top_anime_with_sfw,
            client,
            client.get_top_anime(None, None, None, Some(true), None, None)
        ),
        make_client_test!(
            get_top_anime_with_page,
            client,
            client.get_top_anime(None, None, None, None, Some(1), None)
        ),
        make_client_test!(
            get_top_anime_with_limit,
            client,
            client.get_top_anime(None, None, None, None, None, Some(10))
        ),
        make_client_test!(
            get_top_manga,
            client,
            client.get_top_manga(None, None, None, None)
        ),
        make_client_test!(
            get_top_manga_with_type,
            client,
            client.get_top_manga(Some(MangaType::Manga), None, None, None)
        ),
        make_client_test!(
            get_top_manga_with_filter,
            client,
            client.get_top_manga(None, Some(MangaFilter::Publishing), None, None)
        ),
        make_client_test!(
            get_top_manga_with_page,
            client,
            client.get_top_manga(None, None, Some(1), None)
        ),
        make_client_test!(
            get_top_manga_with_limit,
            client,
            client.get_top_manga(None, None, None, Some(10))
        ),
        make_client_test!(get_top_people, client, client.get_top_people(None, None)),
        make_client_test!(
            get_top_people_with_page,
            client,
            client.get_top_people(Some(1), None)
        ),
        make_client_test!(
            get_top_people_with_limit,
            client,
            client.get_top_people(None, Some(10))
        ),
        make_client_test!(
            get_top_reviews,
            client,
            client.get_top_reviews(None, None, None, None)
        ),
        make_client_test!(
            get_top_reviews_with_type,
            client,
            client.get_top_reviews(Some(ReviewType::Anime), None, None, None)
        ),
        make_client_test!(
            get_top_reviews_with_preliminary,
            client,
            client.get_top_reviews(None, Some(true), None, None)
        ),
        make_client_test!(
            get_top_reviews_with_spoilers,
            client,
            client.get_top_reviews(None, None, Some(true), None)
        ),
        make_client_test!(
            get_top_reviews_with_page,
            client,
            client.get_top_reviews(None, None, None, Some(2))
        ),
    ]
);
