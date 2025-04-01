use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;
use thiserror::Error;

pub mod anime;
pub mod character;
pub mod clubs;
pub mod common;
pub mod genre;
pub mod magazines;
pub mod manga;
pub mod misc;
pub mod people;
pub mod producer;
pub mod random;
pub mod recommendations;
pub mod review;
pub mod schedule;
pub mod seasons;
pub mod top;
pub mod users;

const API_BASE_URL: &str = "https://api.jikan.moe/v4";

#[derive(Error, Debug)]
pub enum JikanError {
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("API response parsing failed: {0}")]
    ParseError(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Resource not found")]
    NotFound,
}

pub struct JikanClient {
    client: Client,
}

impl JikanClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self { client }
    }

    pub async fn get<T>(&self, path: &str) -> Result<T, JikanError>
    where
        T: for<'de> Deserialize<'de>,
    {
        let url = format!("{}{}", API_BASE_URL, path);
        let response = self.client.get(&url).send().await?;

        match response.status() {
            reqwest::StatusCode::OK => {
                let text = response.text().await?;
                serde_json::from_str(&text).map_err(|e| JikanError::ParseError(e.to_string()))
            }
            reqwest::StatusCode::TOO_MANY_REQUESTS => Err(JikanError::RateLimitExceeded),
            reqwest::StatusCode::NOT_FOUND => Err(JikanError::NotFound),
            status => Err(JikanError::BadRequest(format!(
                "Unexpected status code: {}",
                status
            ))),
        }
    }
}

impl Default for JikanClient {
    fn default() -> Self {
        Self::new()
    }
}
