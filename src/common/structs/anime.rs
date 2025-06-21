use crate::{
    common::{response::MalCommonImageResponse, utils::ExternalEntry}, response::MalCommonTypeResponse, structs::{watch::Trailer}, utils::{DateRange, Images, Score, Title}
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anime {
    pub mal_id: u32,
    pub url: String,
    pub images: Images,
    pub trailer: Option<Trailer>,
    pub approved: Option<bool>,
    pub titles: Option<Vec<Title>>,
    pub title: String,
    pub title_english: Option<String>,
    pub title_japanese: Option<String>,
    pub title_synonyms: Option<Vec<String>>,
    pub r#type: Option<String>,
    pub source: Option<String>,
    pub episodes: Option<u32>,
    pub status: Option<String>,
    pub airing: Option<bool>,
    pub aired: Option<DateRange>,
    pub duration: Option<String>,
    pub rating: Option<String>,
    pub score: Option<f32>,
    pub scored_by: Option<u32>,
    pub rank: Option<u32>,
    pub popularity: Option<u32>,
    pub members: Option<u32>,
    pub favorites: Option<u32>,
    pub synopsis: Option<String>,
    pub background: Option<String>,
    pub season: Option<String>,
    pub year: Option<u32>,
    pub broadcast: Option<Broadcast>,
    pub producers: Option<Vec<MalCommonTypeResponse>>,
    pub licensors: Option<Vec<MalCommonTypeResponse>>,
    pub studios: Option<Vec<MalCommonTypeResponse>>,
    pub genres: Option<Vec<MalCommonTypeResponse>>,
    pub explicit_genres: Option<Vec<MalCommonTypeResponse>>,
    pub themes: Option<Vec<MalCommonTypeResponse>>,
    pub demographics: Option<Vec<MalCommonTypeResponse>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Broadcast {
    pub day: Option<String>,
    pub time: Option<String>,
    pub timezone: Option<String>,
    pub string: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeExtended {
    pub mal_id: u32,
    pub url: String,
    pub images: Images,
    pub trailer: Option<Trailer>,
    pub approved: Option<bool>,
    pub titles: Option<Vec<Title>>,
    pub title: String,
    pub title_english: Option<String>,
    pub title_japanese: Option<String>,
    pub title_synonyms: Option<Vec<String>>,
    pub r#type: Option<String>,
    pub source: Option<String>,
    pub episodes: Option<u32>,
    pub status: Option<String>,
    pub airing: Option<bool>,
    pub aired: Option<DateRange>,
    pub duration: Option<String>,
    pub rating: Option<String>,
    pub score: Option<f32>,
    pub scored_by: Option<u32>,
    pub rank: Option<u32>,
    pub popularity: Option<u32>,
    pub members: Option<u32>,
    pub favorites: Option<u32>,
    pub synopsis: Option<String>,
    pub background: Option<String>,
    pub season: Option<String>,
    pub year: Option<u32>,
    pub broadcast: Option<Broadcast>,
    pub producers: Option<Vec<MalCommonTypeResponse>>,
    pub licensors: Option<Vec<MalCommonTypeResponse>>,
    pub studios: Option<Vec<MalCommonTypeResponse>>,
    pub genres: Option<Vec<MalCommonTypeResponse>>,
    pub explicit_genres: Option<Vec<MalCommonTypeResponse>>,
    pub themes: Option<Vec<MalCommonTypeResponse>>,
    pub demographics: Option<Vec<MalCommonTypeResponse>>,
    pub relations: Option<Vec<AnimeRelation>>,
    pub theme: Option<AnimeThemes>,
    pub external: Option<Vec<ExternalEntry>>,
    pub streaming: Option<Vec<ExternalEntry>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaffMember {
    pub person: MalCommonImageResponse,
    pub positions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeThemes {
    pub openings: Vec<String>,
    pub endings: Vec<String>,
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
pub struct AnimeStatistics {
    pub watching: u32,
    pub completed: u32,
    pub on_hold: u32,
    pub dropped: u32,
    pub plan_to_watch: u32,
    pub total: u32,
    pub scores: Vec<Score>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeRelation {
    pub relation: String,
    pub entry: Vec<MalCommonTypeResponse>,
}
