use crate::{
    JikanClient, JikanError,
    enums::{
        anime::{AnimeFilter, AnimeRating, AnimeType},
        manga::{MangaFilter, MangaType},
        reviews::ReviewType,
    },
    response::Response,
    structs::{
        anime::Anime, character::Character, manga::MangaExtended, people::Person,
        reviews::Review,
    },
};

impl JikanClient {
    pub async fn get_top_anime(
        &self,
        r#type: Option<AnimeType>,
        filter: Option<AnimeFilter>,
        rating: Option<AnimeRating>,
        sfw: Option<bool>,
        page: Option<u32>,
        limit: Option<u32>,
    ) -> Result<Response<Vec<Anime>>, JikanError> {
        let mut params = Vec::new();

        if let Some(t_str) = r#type {
            params.push(format!("type={}", t_str.as_str()));
        };

        if let Some(f_str) = filter {
            params.push(format!("filter={}", f_str.as_str()));
        };

        if let Some(r_str) = rating {
            params.push(format!("rating={}", r_str.as_str()));
        };

        if let Some(s) = sfw {
            params.push(format!("sfw={}", s));
        }

        if let Some(p) = page {
            params.push(format!("page={}", p));
        }

        if let Some(l) = limit {
            params.push(format!("limit={}", l));
        }

        let query = if !params.is_empty() {
            format!("?{}", params.join("&"))
        } else {
            String::new()
        };

        self.get(&format!("/top/anime{}", query)).await
    }

    pub async fn get_top_manga(
        &self,
        r#type: Option<MangaType>,
        filter: Option<MangaFilter>,
        page: Option<u32>,
        limit: Option<u32>,
    ) -> Result<Response<Vec<MangaExtended>>, JikanError> {
        let mut params = Vec::new();

        if let Some(t_str) = r#type {
            params.push(format!("type={}", t_str.as_str()));
        };

        if let Some(f_str) = filter {
            params.push(format!("filter={}", f_str.as_str()));
        };

        if let Some(p) = page {
            params.push(format!("page={}", p));
        }

        if let Some(l) = limit {
            params.push(format!("limit={}", l));
        }

        let query = if !params.is_empty() {
            format!("?{}", params.join("&"))
        } else {
            String::new()
        };

        self.get(&format!("/top/manga{}", query)).await
    }

    pub async fn get_top_people(
        &self,
        page: Option<u32>,
        limit: Option<u32>,
    ) -> Result<Response<Vec<Person>>, JikanError> {
        let mut params = Vec::new();
        if let Some(p) = page {
            params.push(format!("page={}", p));
        }
        if let Some(l) = limit {
            params.push(format!("limit={}", l));
        }
        let query = if !params.is_empty() {
            format!("?{}", params.join("&"))
        } else {
            String::new()
        };
        self.get(&format!("/top/people{}", query)).await
    }

    pub async fn get_top_characters(
        &self,
        page: Option<u32>,
        limit: Option<u32>,
    ) -> Result<Response<Vec<Character>>, JikanError> {
        let mut params = Vec::new();
        if let Some(p) = page {
            params.push(format!("page={}", p));
        }
        if let Some(l) = limit {
            params.push(format!("limit={}", l));
        }
        let query = if !params.is_empty() {
            format!("?{}", params.join("&"))
        } else {
            String::new()
        };
        self.get(&format!("/top/characters{}", query)).await
    }

    pub async fn get_top_reviews(
        &self,
        r#type: Option<ReviewType>,
        preliminary: Option<bool>,
        spoilers: Option<bool>,
        page: Option<u32>,
    ) -> Result<Response<Vec<Review>>, JikanError> {
        let mut params = Vec::new();

        if let Some(t_str) = r#type {
            params.push(format!("type={}", t_str.as_str()));
        };

        if let Some(p) = preliminary {
            params.push(format!("preliminary={}", p));
        }
        if let Some(s) = spoilers {
            params.push(format!("spoilers={}", s));
        }
        if let Some(p) = page {
            params.push(format!("page={}", p));
        }
        let query = if !params.is_empty() {
            format!("?{}", params.join("&"))
        } else {
            String::new()
        };
        self.get(&format!("/top/reviews{}", query)).await
    }
}
