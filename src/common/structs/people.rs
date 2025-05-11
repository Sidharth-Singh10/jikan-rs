use crate::{
    structs::{anime::Anime, character::Character, manga::Manga},
    utils::Images,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonAnimePosition {
    pub position: String,
    pub anime: Anime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonMangaPosition {
    pub position: String,
    pub manga: Manga,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonVoiceActingRole {
    pub role: String,
    pub anime: Anime,
    pub character: Character,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub mal_id: u32,
    pub url: String,
    pub website_url: Option<String>,
    pub images: Images,
    pub name: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub alternate_names: Option<Vec<String>>,
    pub birthday: Option<String>,
    pub favorites: Option<u32>,
    pub about: Option<String>,
    pub anime: Option<Vec<PersonAnimePosition>>,
    pub manga: Option<Vec<PersonMangaPosition>>,
    pub voice_acting_roles: Option<Vec<PersonVoiceActingRole>>,
}
