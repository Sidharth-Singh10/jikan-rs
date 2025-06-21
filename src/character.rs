// character.rs
use crate::{
    JikanClient, JikanError,
    enums::{character::CharacterOrder, common::Sort},
    response::Response,
    structs::character::{AnimeEntry, Character, CharacterExtended, MangaEntry, PersonEntry},
    utils::Images,
};

impl JikanClient {
    pub async fn get_character_by_id(&self, id: u32) -> Result<Response<Character>, JikanError> {
        self.get(&format!("/characters/{}", id)).await
    }

    pub async fn get_character_full_by_id(
        &self,
        id: u32,
    ) -> Result<Response<CharacterExtended>, JikanError> {
        self.get(&format!("/characters/{}/full", id)).await
    }

    pub async fn get_character_anime(
        &self,
        id: u32,
    ) -> Result<Response<Vec<AnimeEntry>>, JikanError> {
        self.get(&format!("/characters/{}/anime", id)).await
    }

    pub async fn get_character_manga(
        &self,
        id: u32,
    ) -> Result<Response<Vec<MangaEntry>>, JikanError> {
        self.get(&format!("/characters/{}/manga", id)).await
    }

    pub async fn get_character_voices(
        &self,
        id: u32,
    ) -> Result<Response<Vec<PersonEntry>>, JikanError> {
        self.get(&format!("/characters/{}/voices", id)).await
    }

    pub async fn get_characters(&self) -> Result<Response<Vec<Character>>, JikanError> {
        self.get("/characters").await
    }

    pub async fn get_character_pictures(
        &self,
        id: u32,
    ) -> Result<Response<Vec<Images>>, JikanError> {
        self.get(&format!("/characters/{}/pictures", id)).await
    }

    pub async fn get_character_search(
        &self,
        page: Option<u32>,
        limit: Option<u32>,
        q: Option<String>,
        order_by: Option<CharacterOrder>,
        sort: Option<Sort>,
        letter: Option<String>,
    ) -> Result<Response<Vec<Character>>, JikanError> {
        let mut params: Vec<String> = Vec::new();

        if let Some(p) = page {
            params.push(format!("page={}", p));
        }
        if let Some(l) = limit {
            params.push(format!("limit={}", l));
        }
        if let Some(q) = q {
            params.push(format!("q={}", q));
        }
        if let Some(o) = order_by {
            params.push(format!("order_by={}", o.as_str()));
        }

        if let Some(s) = sort {
            params.push(format!("sort={}", s.as_str()));
        }

        if let Some(lett) = letter {
            if lett.len() != 1 {
                return Err(JikanError::BadRequest(
                    "Letter must be a single character".to_string(),
                ));
            }
            params.push(format!("letter={}", lett));
        }

        let query = if !params.is_empty() {
            format!("?{}", params.join("&"))
        } else {
            String::new()
        };

        self.get(&format!("/characters{}", query)).await
    }
}
