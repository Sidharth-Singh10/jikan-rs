use crate::{response::MalCommonResponse, utils::Images};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Club {
    pub mal_id: i32,
    pub name: String,
    pub url: String,
    pub images: Option<Images>,
    pub members: Option<i32>,
    pub category: Option<String>,
    pub created: Option<String>,
    pub access: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClubMember {
    pub username: String,
    pub url: String,
    pub images: Images,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClubStaff {
    pub url: String,
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClubRelations {
    pub anime: Vec<MalCommonResponse>,
    pub manga: Vec<MalCommonResponse>,
    pub characters: Vec<MalCommonResponse>,
}
