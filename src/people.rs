// people.rs
use crate::{
    JikanClient, JikanError,
    enums::{common::Sort, people::PeopleOrder},
    response::Response,
    structs::people::{Person, PersonAnimePosition, PersonMangaPosition, PersonVoiceActingRole},
    utils::Images,
};

impl JikanClient {
    pub async fn get_person_by_id(&self, id: u32) -> Result<Response<Person>, JikanError> {
        self.get(&format!("/people/{}", id)).await
    }

    pub async fn get_person_by_id_full(&self, id: u32) -> Result<Response<Person>, JikanError> {
        self.get(&format!("/people/{}/full", id)).await
    }

    pub async fn get_person_anime(
        &self,
        id: u32,
    ) -> Result<Response<Vec<PersonAnimePosition>>, JikanError> {
        self.get(&format!("/people/{}/anime", id)).await
    }

    pub async fn get_person_manga(
        &self,
        id: u32,
    ) -> Result<Response<Vec<PersonMangaPosition>>, JikanError> {
        self.get(&format!("/people/{}/manga", id)).await
    }

    pub async fn get_person_voice(
        &self,
        id: u32,
    ) -> Result<Response<Vec<PersonVoiceActingRole>>, JikanError> {
        self.get(&format!("/people/{}/voices", id)).await
    }

    pub async fn get_person_pictures(&self, id: u32) -> Result<Response<Vec<Images>>, JikanError> {
        self.get(&format!("/people/{}/pictures", id)).await
    }

    pub async fn get_people(&self) -> Result<Response<Vec<Person>>, JikanError> {
        self.get("/people").await
    }

    pub async fn get_people_search(
        &self,
        page: Option<u32>,
        limit: Option<u32>,
        query: Option<String>,
        order_by: Option<PeopleOrder>,
        sort: Option<Sort>,
        letter: Option<String>,
    ) -> Result<Response<Vec<Person>>, JikanError> {
        let mut params: Vec<String> = Vec::new();

        if let Some(p) = page {
            params.push(format!("page={}", p));
        }
        if let Some(l) = limit {
            params.push(format!("limit={}", l));
        }
        if let Some(q) = query {
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
        self.get(&format!("/people{}", query)).await
    }
}
