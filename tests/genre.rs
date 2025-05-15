pub mod common;
use crate::common::macs::NamedTestJob;
use jikan_rs::common::enums::genre::GenreFilter;

ratelimited_test_runner!(
    run_ratelimited_tests,
    [
        make_client_test!(get_anime_genres, client, client.get_anime_genres(None)),
        make_client_test!(
            get_anime_genres_genres,
            client,
            client.get_anime_genres(Some(GenreFilter::Genres))
        ),
        make_client_test!(
            get_anime_genres_explicit,
            client,
            client.get_anime_genres(Some(GenreFilter::ExplicitGenres))
        ),
        make_client_test!(
            get_anime_genres_themes,
            client,
            client.get_anime_genres(Some(GenreFilter::Themes))
        ),
        make_client_test!(
            get_anime_genres_demographics,
            client,
            client.get_anime_genres(Some(GenreFilter::Demographics))
        ),
        make_client_test!(get_manga_genres, client, client.get_manga_genres(None)),
        make_client_test!(
            get_manga_genres_genres,
            client,
            client.get_manga_genres(Some(GenreFilter::Genres))
        ),
        make_client_test!(
            get_manga_genres_explicit,
            client,
            client.get_manga_genres(Some(GenreFilter::ExplicitGenres))
        ),
        make_client_test!(
            get_manga_genres_themes,
            client,
            client.get_manga_genres(Some(GenreFilter::Themes))
        ),
        make_client_test!(
            get_manga_genres_demographics,
            client,
            client.get_manga_genres(Some(GenreFilter::Demographics))
        ),
    ]
);
