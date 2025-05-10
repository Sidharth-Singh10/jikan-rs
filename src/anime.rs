// anime.rs
use crate::{
    JikanClient, JikanError,
    character::Character,
    common::{DateRange, Images, Pagination},
    misc::*,
    people::*,
    users::*,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeResponse<T> {
    pub data: T,
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anime {
    pub mal_id: i32,
    pub url: String,
    pub images: Images,
    pub title: String,
    pub start_year: Option<u32>,
    pub title_english: Option<String>,
    pub title_japanese: Option<String>,
    pub episodes: Option<i32>,
    pub status: Option<String>,
    pub score: Option<f32>,
    pub synopsis: Option<String>,
    pub aired: Option<DateRange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeCharacters {
    pub data: Vec<AnimeCharacter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeCharacter {
    pub character: Character,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeStaff {
    pub data: Vec<StaffMember>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaffMember {
    pub person: Person,
    pub positions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeEpisodes {
    pub data: Vec<Episode>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Episode {
    pub mal_id: i32,
    pub url: Option<String>,
    pub title: String,
    pub episode: Option<String>,
    pub aired: Option<String>,
    pub score: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeVideos {
    pub data: Videos,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Videos {
    pub promo: Vec<PromoVideo>,
    pub episodes: Vec<EpisodeVideo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromoVideo {
    pub title: String,
    pub trailer: VideoMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoMeta {
    pub youtube_id: Option<String>,
    pub url: Option<String>,
    pub embed_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodeVideo {
    pub episode: String,
    pub url: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnimeType {
    TV,
    Movie,
    OVA,
    Special,
    ONA,
    Music,
    CM,
    PV,
    TVSpecial,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Sort {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderBy {
    MalId,
    Title,
    StartDate,
    EndDate,
    Episodes,
    Score,
    ScoredBy,
    Rank,
    Popularity,
    Members,
    Favorites,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Status {
    Airing,
    Complete,
    Upcoming,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Rating {
    G,
    Pg,
    Pg13,
    R17,
    R,
    Rx,
}

#[derive(Default, Debug, Clone)]
pub struct SearchParams<'a> {
    pub q: Option<&'a str>,
    pub unapproved: Option<bool>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub type_: Option<AnimeType>,
    pub score: Option<f32>,
    pub min_score: Option<f32>,
    pub max_score: Option<f32>,
    pub status: Option<Status>,
    pub rating: Option<Rating>,
    pub sfw: Option<bool>,
    pub genres: Option<&'a str>,
    pub genres_exclude: Option<&'a str>,
    pub order_by: Option<OrderBy>,
    pub sort: Option<Sort>,
    pub letter: Option<&'a str>,
    pub producers: Option<&'a str>,
    pub start_date: Option<&'a str>,
    pub end_date: Option<&'a str>,
}

impl JikanClient {
    fn format_search_query(query: &str) -> String {
        query
            .to_lowercase()
            .chars()
            .map(|c| match c {
                ' ' => '-',
                c if c.is_alphanumeric() => c,
                _ => ' ',
            })
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join("-")
    }

    pub async fn get_anime(&self, id: i32) -> Result<AnimeResponse<Anime>, JikanError> {
        self.get(&format!("/anime/{}", id)).await
    }

    pub async fn get_anime_search(
        &self,
        params: Option<SearchParams<'_>>,
    ) -> Result<AnimeResponse<Vec<Anime>>, JikanError> {
        let mut query_params = Vec::new();

        if let Some(p) = params {
            if let Some(q) = p.q {
                let formatted_q = Self::format_search_query(q);
                query_params.push(format!("q={}", formatted_q));
            }
            if let Some(u) = p.unapproved {
                query_params.push(format!("unapproved={}", u));
            }
            if let Some(p) = p.page {
                query_params.push(format!("page={}", p));
            }
            if let Some(l) = p.limit {
                query_params.push(format!("limit={}", l));
            }
            if let Some(t) = p.type_ {
                let anime_type = match t {
                    AnimeType::CM => "cm",
                    AnimeType::Movie => "movie",
                    AnimeType::Music => "music",
                    AnimeType::ONA => "ona",
                    AnimeType::OVA => "ova",
                    AnimeType::PV => "pv",
                    AnimeType::Special => "special",
                    AnimeType::TV => "tv",
                    AnimeType::TVSpecial => "tv_special",
                };
                query_params.push(format!("type={}", anime_type));
            }
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
                let status = match st {
                    Status::Airing => "airing",
                    Status::Complete => "complete",
                    Status::Upcoming => "upcoming",
                };
                query_params.push(format!("status={}", status));
            }
            if let Some(r) = p.rating {
                let rating = match r {
                    Rating::G => "g",
                    Rating::Pg => "pg",
                    Rating::Pg13 => "pg13",
                    Rating::R17 => "r17",
                    Rating::R => "r",
                    Rating::Rx => "rx",
                };
                query_params.push(format!("rating={}", rating));
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
                let order_by = match o {
                    OrderBy::EndDate => "end_date",
                    OrderBy::Episodes => "episodes",
                    OrderBy::Favorites => "favorites",
                    OrderBy::MalId => "mal_id",
                    OrderBy::Members => "members",
                    OrderBy::Popularity => "popularity",
                    OrderBy::Rank => "rank",
                    OrderBy::Score => "score",
                    OrderBy::ScoredBy => "scored_by",
                    OrderBy::StartDate => "start_date",
                    OrderBy::Title => "title",
                };
                query_params.push(format!("order_by={}", order_by));
            }
            if let Some(s) = p.sort {
                let sort = match s {
                    Sort::Asc => "asc",
                    Sort::Desc => "desc",
                };
                query_params.push(format!("sort={}", sort));
            }
            if let Some(l) = p.letter {
                query_params.push(format!("letter={}", l));
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

    pub async fn get_anime_full(&self, id: i32) -> Result<AnimeResponse<Anime>, JikanError> {
        self.get(&format!("/anime/{}/full", id)).await
    }

    pub async fn get_anime_characters(&self, id: i32) -> Result<AnimeCharacters, JikanError> {
        self.get(&format!("/anime/{}/characters", id)).await
    }

    pub async fn get_anime_staff(&self, id: i32) -> Result<AnimeStaff, JikanError> {
        self.get(&format!("/anime/{}/staff", id)).await
    }

    pub async fn get_anime_episodes(
        &self,
        id: i32,
        page: Option<u32>,
    ) -> Result<AnimeEpisodes, JikanError> {
        let path = match page {
            Some(p) => format!("/anime/{}/episodes?page={}", id, p),
            None => format!("/anime/{}/episodes", id),
        };
        self.get(&path).await
    }

    pub async fn get_anime_episode(
        &self,
        id: i32,
        episode: i32,
    ) -> Result<AnimeResponse<Episode>, JikanError> {
        self.get(&format!("/anime/{}/episodes/{}", id, episode))
            .await
    }

    pub async fn get_anime_videos(&self, id: i32) -> Result<AnimeVideos, JikanError> {
        self.get(&format!("/anime/{}/videos", id)).await
    }

    pub async fn get_anime_statistics(
        &self,
        id: i32,
    ) -> Result<AnimeResponse<AnimeStatistics>, JikanError> {
        self.get(&format!("/anime/{}/statistics", id)).await
    }

    pub async fn get_anime_themes(
        &self,
        id: i32,
    ) -> Result<AnimeResponse<AnimeThemes>, JikanError> {
        self.get(&format!("/anime/{}/themes", id)).await
    }

    pub async fn get_anime_external(
        &self,
        id: i32,
    ) -> Result<AnimeResponse<Vec<ExternalLink>>, JikanError> {
        self.get(&format!("/anime/{}/external", id)).await
    }

    pub async fn get_anime_streaming(
        &self,
        id: i32,
    ) -> Result<AnimeResponse<Vec<ExternalLink>>, JikanError> {
        self.get(&format!("/anime/{}/streaming", id)).await
    }

    pub async fn get_anime_news(
        &self,
        id: i32,
        page: Option<u32>,
    ) -> Result<AnimeNews, JikanError> {
        let path = match page {
            Some(p) => format!("/anime/{}/news?page={}", id, p),
            None => format!("/anime/{}/news", id),
        };
        self.get(&path).await
    }

    pub async fn get_anime_forum(
        &self,
        id: i32,
        filter: Option<ForumFilter>,
    ) -> Result<AnimeForum, JikanError> {
        let path = match filter {
            Some(f) => format!("/anime/{}/forum?filter={:#?}", id, f),
            None => format!("/anime/{}/forum", id),
        };
        self.get(&path).await
    }

    pub async fn get_anime_videos_episodes(
        &self,
        id: i32,
        page: Option<u32>,
    ) -> Result<AnimeVideosEpisodes, JikanError> {
        let path = match page {
            Some(p) => format!("/anime/{}/videos/episodes?page={}", id, p),
            None => format!("/anime/{}/videos/episodes", id),
        };
        self.get(&path).await
    }

    pub async fn get_anime_pictures(
        &self,
        id: i32,
    ) -> Result<AnimeResponse<Vec<Picture>>, JikanError> {
        self.get(&format!("/anime/{}/pictures", id)).await
    }

    pub async fn get_anime_moreinfo(&self, id: i32) -> Result<AnimeResponse<MoreInfo>, JikanError> {
        self.get(&format!("/anime/{}/moreinfo", id)).await
    }

    pub async fn get_anime_recommendations(
        &self,
        id: i32,
    ) -> Result<AnimeResponse<Vec<Recommendation>>, JikanError> {
        self.get(&format!("/anime/{}/recommendations", id)).await
    }

    pub async fn get_anime_userupdates(
        &self,
        id: i32,
        page: Option<u32>,
    ) -> Result<AnimeUserUpdates, JikanError> {
        let path = match page {
            Some(p) => format!("/anime/{}/userupdates?page={}", id, p),
            None => format!("/anime/{}/userupdates", id),
        };
        self.get(&path).await
    }

    pub async fn get_anime_reviews(
        &self,
        id: i32,
        page: Option<u32>,
        preliminary: Option<bool>,
        spoilers: Option<bool>,
    ) -> Result<AnimeResponse<Vec<Review>>, JikanError> {
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeThemes {
    pub openings: Vec<String>,
    pub endings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeNews {
    pub data: Vec<NewsItem>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeForum {
    pub data: Vec<ForumTopic>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeVideosEpisodes {
    pub data: Vec<EpisodeVideo>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Picture {
    pub images: Images,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoreInfo {
    pub moreinfo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeUserUpdates {
    pub data: Vec<UserUpdate>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeStatistics {
    pub watching: i32,
    pub completed: i32,
    pub on_hold: i32,
    pub dropped: i32,
    pub plan_to_watch: i32,
    pub total: i32,
    pub scores: Vec<Score>,
}
