use crate::structs::anime::Anime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TrailerImages {
    pub image_url: Option<String>,
    pub small_image_url: Option<String>,
    pub medium_image_url: Option<String>,
    pub large_image_url: Option<String>,
    pub maximum_image_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trailer {
    pub youtube_id: Option<String>,
    pub url: Option<String>,
    pub embed_url: Option<String>,
    pub images: Option<TrailerImages>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromoVideo {
    pub title: String,
    pub trailer: Trailer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Episode {
    pub mal_id: u32,
    pub url: Option<String>,
    pub title: String,
    pub title_japanese: Option<String>,
    pub title_romanji: Option<String>,
    pub duration: Option<String>,
    pub aired: Option<String>,
    pub filler: Option<bool>,
    pub recap: Option<bool>,
    pub synopsis: Option<String>,
    pub forum_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchEpisodeEntry {
    pub entry: Anime,
    pub episodes: Vec<Episode>,
    pub region_locked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchPromoEntry {
    pub entry: Anime,
    pub trailer: Trailer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicVideoMeta {
    pub title: String,
    pub author: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicVideo {
    pub title: String,
    pub video: Trailer,
    pub meta: MusicVideoMeta,
}

// Am not sure about strucure of MusicVideo (It's same as docs but I was only getting empty array for all the anime I tried)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Videos {
    pub promo: Vec<PromoVideo>,
    pub episodes: Vec<Episode>,
    pub music_videos: Vec<MusicVideo>,
}
