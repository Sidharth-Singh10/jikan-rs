// people.rs
use crate::{
    JikanClient, JikanError,
    anime::*,
    character::{OrderBy, Sort, *},
    common::{Images, Pagination},
    manga::*,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeopleResponse<T> {
    pub data: T,
    pub pagination: Option<Pagination>,
}

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
    pub mal_id: i32,
    pub url: String,
    pub website_url: Option<String>,
    pub images: Images,
    pub name: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub alternate_names: Option<Vec<String>>,
    pub birthday: Option<String>,
    pub favories: Option<i32>,
    pub about: Option<String>,
    pub anime: Option<Vec<PersonAnimePosition>>,
    pub manga: Option<Vec<PersonMangaPosition>>,
    pub voice_acting_roles: Option<Vec<PersonVoiceActingRole>>,
}

impl JikanClient {
    pub async fn get_person_by_id(&self, id: i32) -> Result<PeopleResponse<Person>, JikanError> {
        self.get(&format!("/people/{}", id)).await
    }

    pub async fn get_person_by_id_full(
        &self,
        id: i32,
    ) -> Result<PeopleResponse<Person>, JikanError> {
        self.get(&format!("/people/{}/full", id)).await
    }

    pub async fn get_person_anime(
        &self,
        id: i32,
    ) -> Result<PeopleResponse<Vec<PersonAnimePosition>>, JikanError> {
        self.get(&format!("/people/{}/anime", id)).await
    }

    pub async fn get_person_manga(
        &self,
        id: i32,
    ) -> Result<PeopleResponse<Vec<PersonMangaPosition>>, JikanError> {
        self.get(&format!("/people/{}/manga", id)).await
    }

    pub async fn get_person_voice(
        &self,
        id: i32,
    ) -> Result<PeopleResponse<Vec<PersonVoiceActingRole>>, JikanError> {
        self.get(&format!("/people/{}/voices", id)).await
    }

    pub async fn get_person_pictures(
        &self,
        id: i32,
    ) -> Result<PeopleResponse<Vec<Images>>, JikanError> {
        self.get(&format!("/people/{}/pictures", id)).await
    }

    pub async fn get_people(&self) -> Result<PeopleResponse<Vec<Person>>, JikanError> {
        self.get("/people").await
    }

    pub async fn get_people_search(
        &self,
        page: Option<i32>,
        limit: Option<i32>,
        query: Option<String>,
        order_by: Option<OrderBy>,
        sort: Option<Sort>,
        letter: Option<String>,
    ) -> Result<PeopleResponse<Vec<Person>>, JikanError> {
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
            let order = match o {
                OrderBy::MalId => "mal_id",
                OrderBy::Name => "name",
                OrderBy::Favorites => "favorites",
            };
            params.push(format!("order_by={}", order));
        }
        if let Some(s) = sort {
            let sort = match s {
                Sort::Asc => "asc",
                Sort::Desc => "desc",
            };
            params.push(format!("sort={}", sort));
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
