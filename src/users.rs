//user.rs
use crate::{
    JikanClient, JikanError,
    enums::users::{Gender, UserHistoryType},
    response::Response,
    structs::{
        clubs::Club,
        recommendation::Recommendation,
        reviews::Review,
        users::{
            Friend, User, UserAbout, UserExtended, UserFavorite, UserHistory, UserStats,
            UserUpdates,
        },
    },
    utils::ExternalEntry,
};

pub struct GetUsersParams {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub q: Option<String>,
    pub gender: Option<Gender>,
    pub location: Option<String>,
    pub max_age: Option<u32>,
    pub min_age: Option<u32>,
}

impl JikanClient {
    pub async fn get_user_full_profile(
        &self,
        username: &str,
    ) -> Result<Response<UserExtended>, JikanError> {
        self.get(&format!("/users/{}/full", username)).await
    }

    pub async fn get_user_profile(&self, username: &str) -> Result<Response<User>, JikanError> {
        self.get(&format!("/users/{}", username)).await
    }

    pub async fn get_user_search(
        &self,
        param: GetUsersParams,
    ) -> Result<Response<Vec<User>>, JikanError> {
        let mut params = Vec::new();

        if let Some(g) = param.gender {
            params.push(format!("gender={}", g.as_str()));
        }

        if let Some(q_str) = param.q {
            params.push(format!("q={}", q_str));
        }

        if let Some(p) = param.page {
            params.push(format!("page={}", p));
        }

        if let Some(l) = param.limit {
            params.push(format!("limit={}", l));
        }

        if let Some(loc) = param.location {
            params.push(format!("location={}", loc));
        }

        if let Some(min_a) = param.min_age {
            params.push(format!("minAge={}", min_a));
        }

        if let Some(max_a) = param.max_age {
            params.push(format!("maxAge={}", max_a));
        }

        let query = if !params.is_empty() {
            format!("?{}", params.join("&"))
        } else {
            String::new()
        };

        self.get(&format!("/users/{}", query)).await
    }

    pub async fn get_user_by_id(&self, id: i32) -> Result<Response<User>, JikanError> {
        //Maybe handle this better?
        self.get(&format!("/users/userbyid/{}", id)).await
    }

    pub async fn get_user_statistics(
        &self,
        username: &str,
    ) -> Result<Response<UserStats>, JikanError> {
        self.get(&format!("/users/{}/statistics", username)).await
    }

    pub async fn get_user_updates(
        &self,
        username: &str,
    ) -> Result<Response<UserUpdates>, JikanError> {
        self.get(&format!("/users/{}/userupdates", username)).await
    }

    pub async fn get_user_friends(
        &self,
        username: &str,
        page: Option<u32>,
    ) -> Result<Response<Vec<Friend>>, JikanError> {
        let mut path = format!("/users/{}/friends", username);

        if let Some(p) = page {
            path = format!("/users/{}/friends?page={}", username, p);
        }

        self.get(&path).await
    }

    pub async fn get_user_reviews(
        &self,
        username: &str,
        page: Option<u32>,
    ) -> Result<Response<Vec<Review>>, JikanError> {
        let mut params = String::new();
        if let Some(p) = page {
            params = format!("?page={}", p);
        }
        self.get(&format!("/users/{}/reviews{}", username, params))
            .await
    }

    pub async fn get_user_recommendations(
        &self,
        username: &str,
        page: Option<u32>,
    ) -> Result<Response<Vec<Recommendation>>, JikanError> {
        let mut params = String::new();
        if let Some(p) = page {
            params = format!("?page={}", p);
        }
        self.get(&format!("/users/{}/recommendations{}", username, params))
            .await
    }

    pub async fn get_user_clubs(
        &self,
        username: &str,
        page: Option<u32>,
    ) -> Result<Response<Vec<Club>>, JikanError> {
        let mut params = String::new();
        if let Some(p) = page {
            params = format!("?page={}", p);
        }
        self.get(&format!("/users/{}/clubs{}", username, params))
            .await
    }

    pub async fn get_user_history(
        &self,
        username: &str,
        r#type: Option<UserHistoryType>,
    ) -> Result<Response<Vec<UserHistory>>, JikanError> {
        let mut params = Vec::new();

        if let Some(g) = r#type {
            params.push(format!("type={}", g.as_str()));
        }

        let query = if !params.is_empty() {
            format!("?{}", params.join("&"))
        } else {
            String::new()
        };

        self.get(&format!("/users/{}/history{}", username, query))
            .await
    }

    pub async fn get_user_favorites(
        &self,
        username: &str,
    ) -> Result<Response<UserFavorite>, JikanError> {
        self.get(&format!("/users/{}/favorites", username)).await
    }

    pub async fn get_user_external(
        &self,
        username: &str,
    ) -> Result<Response<Vec<ExternalEntry>>, JikanError> {
        self.get(&format!("/users/{}/external", username)).await
    }

    pub async fn get_user_about(&self, username: &str) -> Result<Response<UserAbout>, JikanError> {
        self.get(&format!("/users/{}/about", username)).await
    }
}
