// anime.rs
use crate::{
    JikanClient, JikanError,
    common::structs::anime::AnimeExtended,
    enums::{
        anime::{AnimeOrder, AnimeRating, AnimeStatus, AnimeType},
        common::Sort,
        forum::ForumFilter,
    },
    format_search_query,
    response::Response,
    structs::{
        anime::{
            Anime, AnimeForum, AnimeRelation, AnimeStatistics, AnimeThemes, MoreInfo, StaffMember,
        },
        character::CharacterRole,
        forum::NewsItem,
        recommendation::RecommendationAlt,
        reviews::Review,
        users::UserUpdate,
        watch::{Episode, Videos},
    },
    utils::{ExternalEntry, Images},
};

#[derive(Default)]
pub struct SearchParams<'a> {
    pub q: Option<String>,
    pub unapproved: Option<bool>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub type_: Option<AnimeType>,
    pub score: Option<f32>,
    pub min_score: Option<f32>,
    pub max_score: Option<f32>,
    pub status: Option<AnimeStatus>,
    pub rating: Option<AnimeRating>,
    pub sfw: Option<bool>,
    pub genres: Option<&'a str>,
    pub genres_exclude: Option<&'a str>,
    pub order_by: Option<AnimeOrder>,
    pub sort: Option<Sort>,
    pub letter: Option<&'a str>,
    pub producers: Option<&'a str>,
    pub start_date: Option<&'a str>,
    pub end_date: Option<&'a str>,
}

impl JikanClient {
    pub async fn get_anime(&self, id: i32) -> Result<Response<Anime>, JikanError> {
        self.get(&format!("/anime/{}", id)).await
    }

    pub async fn get_anime_search(
        &self,
        params: Option<SearchParams<'_>>,
    ) -> Result<Response<Vec<Anime>>, JikanError> {
        let mut query_params = Vec::new();

        if let Some(p) = params {
            if let Some(q) = p.q {
                let formatted_q = format_search_query(q);
                query_params.push(format!("q={}", formatted_q));
            }

            if let Some(u) = p.unapproved {
                if u {
                    query_params.push("unapproved".to_string());
                }
            }

            if let Some(p) = p.page {
                query_params.push(format!("page={}", p));
            }

            if let Some(l) = p.limit {
                query_params.push(format!("limit={}", l));
            }

            if let Some(t) = p.type_ {
                query_params.push(format!("type={}", t.as_str()));
            }

            #[allow(non_snake_case)]
            match p.score {
                //* this is due the fact that the query may not have 'score' alongside 'min_score' or 'max_score'
                Some(score) => query_params.push(format!("score={}", score)),
                None => {
                    if let Some(min) = p.min_score {
                        query_params.push(format!("min_score={}", min));
                    }

                    if let Some(max) = p.max_score {
                        query_params.push(format!("max_score={}", max));
                    }
                }
            }

            if let Some(st) = p.status {
                query_params.push(format!("status={}", st.as_str()));
            }

            if let Some(r) = p.rating {
                query_params.push(format!("rating={}", r.as_str()));
            }

            if let Some(s) = p.sfw {
                query_params.push(format!("sfw={}", s));
            }

            if let Some(g) = p.genres {
                query_params.push(format!("genres={}", g));
            }

            if let Some(ge) = p.genres_exclude {
                query_params.push(format!("genres_exclude={}", ge));
            }

            if let Some(o) = p.order_by {
                query_params.push(format!("order_by={}", o.as_str()));
            }

            if let Some(s) = p.sort {
                query_params.push(format!("sort={}", s.as_str()));
            }

            if let Some(lett) = p.letter {
                if lett.len() != 1 {
                    return Err(JikanError::BadRequest(
                        "Letter must be a single character".to_string(),
                    ));
                }
                query_params.push(format!("letter={}", lett));
            }

            if let Some(p) = p.producers {
                query_params.push(format!("producers={}", p));
            }

            if let Some(s) = p.start_date {
                query_params.push(format!("start_date={}", s));
            }

            if let Some(e) = p.end_date {
                query_params.push(format!("end_date={}", e));
            }
        }

        let query = format!("?{}", query_params.join("&"));
        self.get(&format!("/anime{}", query)).await
    }

    pub async fn get_anime_full(&self, id: i32) -> Result<Response<AnimeExtended>, JikanError> {
        self.get(&format!("/anime/{}/full", id)).await
    }

    pub async fn get_anime_characters(
        &self,
        id: i32,
    ) -> Result<Response<Vec<CharacterRole>>, JikanError> {
        self.get(&format!("/anime/{}/characters", id)).await
    }

    pub async fn get_anime_staff(&self, id: i32) -> Result<Response<Vec<StaffMember>>, JikanError> {
        self.get(&format!("/anime/{}/staff", id)).await
    }

    pub async fn get_anime_episodes(
        &self,
        id: i32,
        page: Option<u32>,
    ) -> Result<Response<Vec<Episode>>, JikanError> {
        let mut path = format!("/anime/{}/episodes", id);

        if let Some(p) = page {
            path = format!("/anime/{}/episodes?page={}", id, p);
        };

        self.get(&path).await
    }

    pub async fn get_anime_episode(
        &self,
        id: i32,
        episode: i32,
    ) -> Result<Response<Episode>, JikanError> {
        self.get(&format!("/anime/{}/episodes/{}", id, episode))
            .await
    }

    pub async fn get_anime_videos(&self, id: i32) -> Result<Response<Videos>, JikanError> {
        self.get(&format!("/anime/{}/videos", id)).await
    }

    pub async fn get_anime_statistics(
        &self,
        id: i32,
    ) -> Result<Response<AnimeStatistics>, JikanError> {
        self.get(&format!("/anime/{}/statistics", id)).await
    }

    pub async fn get_anime_themes(&self, id: i32) -> Result<Response<AnimeThemes>, JikanError> {
        self.get(&format!("/anime/{}/themes", id)).await
    }

    pub async fn get_anime_external(
        &self,
        id: i32,
    ) -> Result<Response<Vec<ExternalEntry>>, JikanError> {
        self.get(&format!("/anime/{}/external", id)).await
    }

    pub async fn get_anime_streaming(
        &self,
        id: i32,
    ) -> Result<Response<Vec<ExternalEntry>>, JikanError> {
        self.get(&format!("/anime/{}/streaming", id)).await
    }

    pub async fn get_anime_news(
        &self,
        id: i32,
        page: Option<u32>,
    ) -> Result<Response<Vec<NewsItem>>, JikanError> {
        let mut path = format!("/anime/{}/news", id);

        if let Some(p) = page {
            path = format!("/anime/{}/news?page={}", id, p);
        }

        self.get(&path).await
    }

    pub async fn get_anime_forum(
        &self,
        id: i32,
        filter: Option<ForumFilter>,
    ) -> Result<AnimeForum, JikanError> {
        let mut path = format!("/anime/{}/forum", id);

        if let Some(p) = filter {
            path = format!("/anime/{}/forum?filter={}", id, p.as_str());
        }

        self.get(&path).await
    }

    pub async fn get_anime_videos_episodes(
        &self,
        id: i32,
        page: Option<u32>,
    ) -> Result<Response<Vec<Episode>>, JikanError> {
        let mut path = format!("/anime/{}/videos/episodes", id);

        if let Some(p) = page {
            path = format!("/anime/{}/videos/episodes?page={}", id, p);
        }

        self.get(&path).await
    }

    pub async fn get_anime_pictures(&self, id: i32) -> Result<Response<Vec<Images>>, JikanError> {
        self.get(&format!("/anime/{}/pictures", id)).await
    }

    pub async fn get_anime_moreinfo(&self, id: i32) -> Result<Response<MoreInfo>, JikanError> {
        self.get(&format!("/anime/{}/moreinfo", id)).await
    }

    pub async fn get_anime_recommendations(
        &self,
        id: i32,
    ) -> Result<Response<Vec<RecommendationAlt>>, JikanError> {
        self.get(&format!("/anime/{}/recommendations", id)).await
    }

    pub async fn get_anime_userupdates(
        &self,
        id: i32,
        page: Option<u32>,
    ) -> Result<Response<Vec<UserUpdate>>, JikanError> {
        let mut path = format!("/anime/{}/userupdates", id);

        if let Some(p) = page {
            path = format!("/anime/{}/userupdates?page={}", id, p);
        }

        self.get(&path).await
    }

    pub async fn get_anime_reviews(
        &self,
        id: i32,
        page: Option<u32>,
        preliminary: Option<bool>,
        spoilers: Option<bool>,
    ) -> Result<Response<Vec<Review>>, JikanError> {
        let mut params = Vec::new();

        if let Some(p) = page {
            params.push(format!("page={}", p));
        }
        if let Some(pr) = preliminary {
            params.push(format!("preliminary={}", pr));
        }
        if let Some(sp) = spoilers {
            params.push(format!("spoilers={}", sp));
        }

        let query = if !params.is_empty() {
            format!("?{}", params.join("&"))
        } else {
            String::new()
        };

        self.get(&format!("/anime/{}/reviews{}", id, query)).await
    }

    pub async fn get_anime_relations(
        &self,
        id: i32,
    ) -> Result<Response<Vec<AnimeRelation>>, JikanError> {
        self.get(&format!("/anime/{}/relations", id)).await
    }
}
