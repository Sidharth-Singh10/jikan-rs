use crate::{
    common::utils::ExternalEntry,
    response::MalCommonTypeResponse,
    structs::{anime::Anime, character::Character, manga::Manga, people::Person},
    utils::Images,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub url: String,
    pub images: Option<Images>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAnimeInfo {
    pub days_watched: f32,
    pub mean_score: f32,
    pub watching: u32,
    pub completed: u32,
    pub on_hold: u32,
    pub dropped: u32,
    pub plan_to_watch: u32,
    pub total_entries: u32,
    pub rewatched: u32,
    pub episodes_watched: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMangaInfo {
    pub days_read: f32,
    pub mean_score: f32,
    pub reading: u32,
    pub completed: u32,
    pub on_hold: u32,
    pub dropped: u32,
    pub plan_to_read: u32,
    pub total_entries: u32,
    pub reread: u32,
    pub chapters_read: u32,
    pub volumes_read: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStats {
    pub anime: UserAnimeInfo,
    pub manga: UserMangaInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUpdate {
    pub user: User,
    pub score: Option<u32>,
    pub status: String,
    pub chapters_read: Option<u32>,
    pub chapters_total: Option<u32>,
    pub episodes_seen: Option<u32>,
    pub episodes_total: Option<u32>,
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAnimeUpdates {
    pub entry: Anime,
    pub score: Option<u32>,
    pub status: String,
    pub episodes_seen: Option<u32>,
    pub episodes_total: Option<u32>,
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMangaUpdates {
    pub entry: Manga,
    pub score: Option<u32>,
    pub status: String,
    pub chapters_read: Option<u32>,
    pub chapters_total: Option<u32>,
    pub volumes_read: Option<u32>,
    pub volumes_total: Option<u32>,
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Friend {
    user: User,
    last_online: Option<String>,
    friends_since: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserHistory {
    pub entry: MalCommonTypeResponse,
    pub increment: Option<u32>,
    pub date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFavorite {
    pub anime: Vec<Anime>,
    pub manga: Vec<Manga>,
    pub characters: Vec<Character>,
    pub people: Vec<Person>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUpdates {
    anime: Vec<UserAnimeUpdates>,
    manga: Vec<UserMangaUpdates>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAbout {
    pub about: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserExtended {
    mal_id: u32,
    username: String,
    url: String,
    images: Option<Images>,
    last_online: String,
    gender: Option<String>,
    birthday: Option<String>,
    location: Option<String>,
    joined: String,
    statistics: Option<UserStats>,
    external: Option<Vec<ExternalEntry>>,
}
