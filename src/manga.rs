use crate::{
    JikanClient, JikanError,
    enums::{
        common::Sort,
        forum::ForumFilter,
        manga::{MangaOrder, MangaStatus, MangaType},
    },
    format_search_query,
    response::Response,
    structs::{
        character::CharacterRole,
        forum::{ForumTopic, NewsItem},
        manga::{Manga, MangaRelation, MangaStatistics, MoreInfo},
        recommendation::RecommendationAlt,
        reviews::Review,
        users::UserUpdate,
    },
    utils::{ExternalEntry, Images},
};

#[derive(Default)]
pub struct SearchParams<'a> {
    pub q: Option<String>,
    pub unapproved: Option<bool>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub type_: Option<MangaType>,
    pub score: Option<f32>,
    pub min_score: Option<f32>,
    pub max_score: Option<f32>,
    pub status: Option<MangaStatus>,
    pub sfw: Option<bool>,
    pub genres: Option<&'a str>,
    pub genres_exclude: Option<&'a str>,
    pub magazines: Option<&'a str>,
    pub order_by: Option<MangaOrder>,
    pub sort: Option<Sort>,
    pub letter: Option<&'a str>,
    pub producers: Option<&'a str>,
    pub start_date: Option<&'a str>,
    pub end_date: Option<&'a str>,
}

impl JikanClient {
    pub async fn get_manga(&self, id: u32) -> Result<Response<Manga>, JikanError> {
        self.get(&format!("/manga/{}", id)).await
    }

    pub async fn get_manga_full(&self, id: u32) -> Result<Response<Manga>, JikanError> {
        self.get(&format!("/manga/{}/full", id)).await
    }

    pub async fn get_manga_characters(
        &self,
        id: u32,
    ) -> Result<Response<Vec<CharacterRole>>, JikanError> {
        self.get(&format!("/manga/{}/characters", id)).await
    }

    pub async fn get_manga_news(
        &self,
        id: u32,
        page: Option<u32>,
    ) -> Result<Response<Vec<NewsItem>>, JikanError> {
        let mut path = format!("/manga/{}/news", id);

        if let Some(p) = page {
            path = format!("/manga/{}/news?page={}", id, p);
        }

        self.get(&path).await
    }

    pub async fn get_manga_forum(
        &self,
        id: u32,
        filter: Option<ForumFilter>,
    ) -> Result<Response<Vec<ForumTopic>>, JikanError> {
        let mut path = format!("/manga/{}/forum", id);

        if let Some(p) = filter {
            path = format!("/manga/{}/forum?filter={}", id, p.as_str());
        };

        self.get(&path).await
    }

    pub async fn get_manga_pictures(&self, id: u32) -> Result<Response<Vec<Images>>, JikanError> {
        self.get(&format!("/manga/{}/pictures", id)).await
    }

    pub async fn get_manga_statistics(
        &self,
        id: u32,
    ) -> Result<Response<MangaStatistics>, JikanError> {
        self.get(&format!("/manga/{}/statistics", id)).await
    }

    pub async fn get_manga_moreinfo(&self, id: u32) -> Result<Response<MoreInfo>, JikanError> {
        self.get(&format!("/manga/{}/moreinfo", id)).await
    }

    pub async fn get_manga_recommendations(
        &self,
        id: u32,
    ) -> Result<Response<Vec<RecommendationAlt>>, JikanError> {
        self.get(&format!("/manga/{}/recommendations", id)).await
    }

    pub async fn get_manga_userupdates(
        &self,
        id: u32,
        page: Option<u32>,
    ) -> Result<Response<Vec<UserUpdate>>, JikanError> {
        let mut path = format!("/manga/{}/userupdates", id);

        if let Some(p) = page {
            path = format!("/manga/{}/userupdates?page={}", id, p);
        }

        self.get(&path).await
    }

    pub async fn get_manga_reviews(
        &self,
        id: u32,
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

        self.get(&format!("/manga/{}/reviews{}", id, query)).await
    }

    pub async fn get_manga_relations(
        &self,
        id: u32,
    ) -> Result<Response<Vec<MangaRelation>>, JikanError> {
        self.get(&format!("/manga/{}/relations", id)).await
    }

    pub async fn get_manga_external(
        &self,
        id: u32,
    ) -> Result<Response<Vec<ExternalEntry>>, JikanError> {
        self.get(&format!("/manga/{}/external", id)).await
    }

    pub async fn get_manga_search(
        &self,
        params: Option<SearchParams<'_>>,
    ) -> Result<Response<Vec<Manga>>, JikanError> {
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

            if let Some(m) = p.magazines {
                query_params.push(format!("magazines={}", m));
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
        self.get(&format!("/manga{}", query)).await
    }
}
