use crate::{
    response::MalCommonTypeResponse,
    utils::{DateRange, Images, Score, Title},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manga {
    pub mal_id: u32,
    pub url: String,
    pub images: Images,
    pub title: String,
    pub title_english: Option<String>,
    pub title_japanese: Option<String>,
    pub chapters: Option<u32>,
    pub volumes: Option<u32>,
    pub status: Option<String>,
    pub start_year: Option<u32>,
    pub score: Option<f32>,
    pub synopsis: Option<String>,
    pub published: Option<DateRange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaExtended {
    pub mal_id: u32,
    pub url: String,
    pub images: Images,
    pub approved: bool,
    pub titles: Vec<Title>,
    pub title: String,
    pub title_english: Option<String>,
    pub title_japanese: Option<String>,
    pub r#type: String,
    pub chapters: Option<u32>,
    pub volumes: Option<u32>,
    pub status: String,
    pub publishing: bool,
    pub published: DateRange,
    pub score: Option<f32>,
    pub scored_by: Option<u32>,
    pub rank: Option<u32>,
    pub popularity: Option<u32>,
    pub members: Option<u32>,
    pub favorites: Option<u32>,
    pub synopsis: Option<String>,
    pub background: Option<String>,
    pub authors: Vec<MalCommonTypeResponse>,
    pub serializations: Vec<MalCommonTypeResponse>,
    pub genres: Vec<MalCommonTypeResponse>,
    pub explicit_genres: Vec<MalCommonTypeResponse>,
    pub themes: Vec<MalCommonTypeResponse>,
    pub demographics: Vec<MalCommonTypeResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaRelation {
    pub relation: String,
    pub entry: Vec<MalCommonTypeResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoreInfo {
    pub moreinfo: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaStatistics {
    pub reading: u32,
    pub completed: u32,
    pub on_hold: u32,
    pub dropped: u32,
    pub plan_to_read: u32,
    pub total: u32,
    pub scores: Vec<Score>,
}
