use crate::{structs::users::User, utils::Images};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationEntry {
    pub mal_id: i32,
    pub url: String,
    pub images: Images,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub mal_id: String,
    pub entry: Vec<RecommendationEntry>,
    pub content: String,
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationAlt {
    pub entry: RecommendationEntry,
    pub votes: i32,
}
