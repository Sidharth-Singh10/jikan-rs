use crate::{JikanClient, JikanError, common::Pagination};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewResponse<T> {
    pub data: T,
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeReviewCollection {
    pub data: Vec<ReviewEntry>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaReviewCollection {
    pub data: Vec<ReviewEntry>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewEntry {
    pub user: UserMeta,
    pub mal_id: i32,
    pub url: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub reactions: Option<ReviewReactions>,
    pub date: String,
    pub review: String,
    pub score: i32,
    pub tags: Vec<String>,
    pub is_spoiler: bool,
    pub is_preliminary: bool,
    pub episodes_watched: Option<i32>,
    pub chapters_read: Option<i32>,
    // Define entry field for either anime or manga
    pub entry: EntryMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryMeta {
    pub mal_id: i32,
    pub url: String,
    pub images: Images,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMeta {
    pub username: String,
    pub url: String,
    pub images: UserImages,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Images {
    pub jpg: JpgImage,
    pub webp: Option<WebpImage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JpgImage {
    pub image_url: Option<String>,
    pub small_image_url: Option<String>,
    pub large_image_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebpImage {
    pub image_url: Option<String>,
    pub small_image_url: Option<String>,
    pub large_image_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserImages {
    pub jpg: UserJpgImage,
    pub webp: Option<UserWebpImage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserJpgImage {
    pub image_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserWebpImage {
    pub image_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewReactions {
    pub overall: i32,
    pub nice: i32,
    pub love_it: i32,
    pub funny: i32,
    pub confusing: i32,
    pub informative: i32,
    pub well_written: i32,
    pub creative: i32,
}

impl JikanClient {
    pub async fn get_recent_anime_reviews(
        &self,
        page: Option<u32>,
        preliminary: Option<bool>,
        spoilers: Option<bool>,
    ) -> Result<AnimeReviewCollection, JikanError> {
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

        self.get(&format!("/reviews/anime{}", query)).await
    }

    pub async fn get_recent_manga_reviews(
        &self,
        page: Option<u32>,
        preliminary: Option<bool>,
        spoilers: Option<bool>,
    ) -> Result<MangaReviewCollection, JikanError> {
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

        self.get(&format!("/reviews/manga{}", query)).await
    }
}
