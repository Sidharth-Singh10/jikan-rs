use crate::{structs::users::User, utils::Images};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewReactions {
    pub overall: u32,
    pub nice: u32,
    pub love_it: u32,
    pub funny: u32,
    pub confusing: u32,
    pub informative: u32,
    pub well_written: u32,
    pub creative: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryMeta {
    pub mal_id: u32,
    pub url: String,
    pub images: Images,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    pub mal_id: u32,
    pub url: String,
    pub r#type: String,
    pub reactions: Option<ReviewReactions>,
    pub date: String,
    pub review: String,
    pub score: u32,
    pub tags: Vec<String>,
    pub entry: Option<EntryMeta>,
    pub user: Option<User>,
    pub is_spoiler: bool,
    pub is_preliminary: bool,
    pub episodes_watched: Option<u32>,
    pub chapters_read: Option<u32>,
}
